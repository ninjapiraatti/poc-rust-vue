<template>
	<div v-if="users.length">
		<div class="table-responsive">
			<table class="table table-striped" :class='$colorScheme.table'>
				<thead>
					<tr>
						<th scope="col"></th>
						<th scope="col">Name</th>
						<th scope="col" class='text-center'>Mandatory skills</th>
						<th scope="col">Matched skills</th>
						<th scope="col" class='text-center'>Tier</th>
						<th scope="col" class='text-center'>Available</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="(user, index) in users" :key="user.user_id" :class="`tier tier-${user.tier}`">
						<th scope="row"><div class="tier-circle shadow-sm" :style='{ zIndex: users.length - index }'>{{ index + 1 }}</div></th>
						<td>
							<button v-on:click='showMatch(user)' class='btn btn-unstyled'>
								{{ user.user_first_name }} {{ user.user_last_name }} 
							</button>
							<i v-if='user.user_favorite' class='bi-star-fill text-yellow ms-2 float-end'></i>
						</td>
						<td class='text-center'>{{ user.hasMandatory }}</td>
						<td><span class="badge badge-skill me-2" v-for="skill in user.skills" :key="skill.skill_id">{{ skill.skill_label }}</span></td>
						<td class='text-center'>{{ user.tier }}</td>
						<td class='text-center'>{{ user.isAvailable }}</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>

<script>
	import MatchContent from './MatchContent.vue'

	export default {
		name: 'VMatchesForProject',

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
				return user.skills.every(match => match.required_load >= match.user_load)
			},

			showMatch(props) {
				this.$modal({
					title: 'Match',
					component: MatchContent,
					props,
				})
			},
		},
	}
</script>