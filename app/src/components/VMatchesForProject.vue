<template>
	<div v-if="users.length">
		<table class="table table-striped mb-0" :class='$colorScheme.table'>
			<thead>
				<tr>
					<th scope="col" class='d-none d-sm-table-cell'>
						<div class='visually-hidden'>Match order</div>
					</th>
					<th scope="col">Name</th>
					<th scope="col" class='d-none d-sm-table-cell'>Skills</th>
					<th scope="col" class='text-center'>
						<span class='d-none d-sm-inline'>Mandatory skills</span>
						<span class='d-sm-none'>Skills</span>
					</th>
					<th scope="col" class='text-center'>Available</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="(user, index) in users" :key="user.user_id" :class="`tier tier-${user.tier}`">
					<th scope="row" class='d-none d-sm-table-cell'><div class="tier-circle shadow-sm" :style='{ zIndex: users.length - index }'>{{ index + 1 }}</div></th>
					<td>
						<button v-on:click='showMatch(user)' class='btn btn-unstyled'>
							{{ user.user_first_name }} {{ user.user_last_name }} 
						</button>
						<i v-if='user.user_favorite' class='bi-star-fill text-yellow ms-2 float-end'></i>
					</td>
					<td class='d-none d-sm-table-cell align-middle'>
						<div class='hstack gap-2 flex-wrap'>
							<VSkillBadge
								v-for="skill in user.skills"
								:key="skill.skill_id"
								:label='skill.skill_label'
								:mandatory='skill.skill_mandatory'
								:percentage='skill.skill_percentage'
							/>
						</div>
					</td>
					<td class='text-center align-middle'>
						<i v-if='user.hasMandatory' class='bi-check-lg' title='Mandatory skills'></i>
					</td>
					<td class='text-center align-middle'>
						<i v-if='user.isAvailable' class='bi-check-lg' title='Available'></i>
					</td>
				</tr>
			</tbody>
		</table>
	</div>
</template>

<script>
	import MatchContent from '@components/MatchContent.vue'
	import VSkillBadge from '@components/VSkillBadge.vue'

	export default {
		name: 'VMatchesForProject',

		components: {
			VSkillBadge,
		},

		props: {
			project: {
				type: Object,
				required: true,
			},
			matches: {
				type: Object,
				required: true,
			},
		},

		computed: {
			users() {
				if (this.matches) {
					const users = Object.values(this.matches).map(user => {
						user.hasMandatory = this.hasMandatorySkills(user)
						user.isAvailable = this.isAvailable(user)
						user.tier = this.tier(user)
						return user
					})

					return users.sort((a, b) => {
						if (a.tier != b.tier) return a.tier < b.tier ? -1 : 1
						if (a.skills.length != b.skills.length) return a.skills.length > b.skills.length ? -1 : 1
						return 0
					})
				}
			},

			mandatorySkills() {
				const skills = this.project.needs.reduce((skills, need) => {
					const needSkills = need.skills.reduce((skills, skill) => {
						if (skill.mandatory) skills.push(skill.skill_label)
						return skills
					}, [])

					return skills.concat(needSkills)
				}, [])

				return skills
			},
		},

		methods: {
			tier(user) {
				return user.hasMandatory
					? user.isAvailable ? 1 : 2
					: user.isAvailable ? 3 : 4
			},

			hasMandatorySkills(user) {
				if (!this.mandatorySkills.length) return false
				return this.mandatorySkills.every(skill => {
					return user.skills.some(match => {
						return match.skill_label == skill
							&& match.user_index >= match.required_index
							&& match.user_years >= match.required_minyears
							&& (match.user_years <= match.required_maxyears || !match.required_maxyears)
					})
				})
			},

			isAvailable(user) {
				return user.skills.every(match => match.required_load <= 100 - match.user_load)
			},

			showMatch(props) {
				this.$modal({
					title: 'Match',
					component: MatchContent,
					size: 'lg',
					props,
				})
			},
		},
	}
</script>
