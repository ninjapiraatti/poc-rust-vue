<template>
	<div class='card shadow' :class='$colorScheme.card'>
		<div class='card-header'>
			<div class="d-flex justify-content-between align-items-center flex-wrap">
				<h1 class="h3 flex-grow-1 mb-0">Skills &amp; Categories</h1>
				<button class="btn btn-primary gradient" v-on:click="editCategory()">New category</button>
			</div>
		</div>
		<div class='card-body'>
			<div v-if='categories.length'>
				<table class="table table-striped table-stack-mobile mb-0 table-lg" :class='$colorScheme.table'>
					<thead>
						<tr>
							<th scope="col" class='ps-3'>Category</th>
							<th scope="col">Skills</th>
						</tr>
					</thead>
					<tbody>
						<tr v-for="category in categories" :key="category.id">
							<td class='context'>
								<div class='table-stack-mobile-cell'>
									<div class='d-flex justify-content-between'>
										<div>{{ category.label }}</div>
										<div class='context-actions hstack gap-1 justify-content-end'>
											<button class='btn btn-unstyled px-1 rounded' v-on:click="editSkill(category)"><i class="bi-plus-circle-fill" title='Add skill'></i></button>
											<button class='btn btn-unstyled px-1 rounded' v-on:click="editCategory(category)"><i class="bi-pencil-fill" title='Edit category'></i></button>
											<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('skill.category', category)"><i class="bi-trash-fill" title='Delete category'></i></button>
										</div>
									</div>
								</div>
							</td>
							<td>
								<div class='table-stack-mobile-cell'>
									<ul class='list-group list-group-flush list-group-transparent my-n2'>
										<li v-for="skill in filterSkills(category.id)" :key="skill" class='list-group-item d-flex justify-content-between ps-4 pe-0 px-md-0 context'>
											<div>{{ skill.label }} <small class="fw-light text-muted">({{ getSkillScopeLabel(skill.skillscope_id) }})</small></div>
											<div class='context-actions hstack gap-1 justify-content-end'>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="editSkill(skill)"><i class="bi-pencil-fill" title='Edit skill'></i></button>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('skill', skill)"><i class="bi-trash-fill" title='Delete skill'></i></button>
											</div>
										</li>
									</ul>
								</div>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
			<div v-else>
				<div class='fs-3 fw-light text-muted text-center p-4'>No skills</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FormSkill from '@forms/FormSkill.vue'
	import FormSkillCategory from '@forms/FormSkillCategory.vue'

	export default {
		name: 'AdminListSkills',
		methods: {
			filterSkills(id) {
				return this.skills.filter(skill => skill.skillcategory_id == id)
			},
			getSkillScopeLabel(id) {
				if (id && this.scopes.length) {
					var scope = this.scopes.find(skillScope => skillScope.id == id)
					return scope.label
				}
			},
			async editCategory(props = {}) {
				const result = await this.$modal({
					title: props.id ? `Edit category: ${props.label}` : 'Add category',
					component: FormSkillCategory,
					props,
				})

				if (result) this.$store.dispatch('getSkillCategories')
			},
			async editSkill(props = {}) {
				const result = await this.$modal({
					title: props.skillcategory_id ? `Edit skill: ${props.label}` : `Add skill to ${props.label}`,
					component: FormSkill,
					props: props.skillcategory_id ? props : { skillcategory_id: props.id }
				})

				if (result) {
					this.$store.dispatch('getSkillCategories')
					this.$store.dispatch('getSkills')
				}
			},
			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				
				if (success) {
					switch (type) {
						case 'skill':
							this.$store.dispatch('getSkills')
							break

						case 'skill.category':
							this.$store.dispatch('getSkillCategories')
							break
					}
				}
			},
		},
		computed: {
			categories() {
				return this.$store.state.skillCategories
			},
			scopes() {
				return this.$store.state.skillScopes
			},
			skills() {
				return this.$store.state.skills
			},
		},
		activated() {
			this.$store.dispatch('getSkills')
			this.$store.dispatch('getSkillCategories')
			this.$store.dispatch('getSkillScopes')
		}
	}
</script>
