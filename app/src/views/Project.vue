<template>
	<div class="container mt-4" v-if='project'>
		<div class="row gx-4">
			<div class="col-md-4">
                <div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
                	<h2>{{ project.name }}</h2>
					<router-link :to='{ name: "page-project-edit" }' class='btn btn-link p-0 border-0'><i class="bi-pencil-fill p-1"></i></router-link>
					<button type='button' class='btn btn-link p-0 border-0' @click="confirmDeleteProject()"><i class="bi-trash-fill p-1"></i></button>
                </div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<div class="d-flex flex-row justify-content-between align-items-start">
						<h2>Needs</h2>
						<router-link :to='{ name: "page-project-need" }' class="btn btn-gradient">Add need</router-link>
					</div>
					<div class="mt-3" v-for="need in project.needs" :key="need.id">
						<hr />
						<div class="d-flex flex-row justify-content-between align-items-baseline mb-3">
							<h5>{{ need.count_of_users}} from {{ need.begin_time }} at percentage: {{ need.percentage}}</h5>
							<div class="btn-group" role="group" aria-label="Need actions">
								<router-link :to='{ name: "page-project-skill", params: { needId: need.id } }'><i class="bi-plus-circle-fill p-1"></i></router-link>
								<router-link :to='{ name: "page-project-need", params: { needId: need.id } }'><i class="bi-pencil-fill p-1"></i></router-link>
								<button type='button' class='btn btn-link p-0 border-0' @click="confirmDeleteNeed(need.id)"><i class="bi-trash-fill p-1"></i></button>
							</div>
						</div>
						<table class="table table-dark table-striped text-light mb-4">
							<thead>
								<tr>
									<th scope="col">Skill</th>
									<th scope="col">Mandatory</th>
									<th scope="col">Min level</th>
									<th scope="col">Min years</th>
									<th scope="col">Max years</th>
									<th scope="col">Actions</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="skill in need.skills" :key="skill.id">
									<td>{{ skill.label }}</td>
									<td>{{ skill.mandatory }}</td>
									<td>{{ skill.level }}</td>
									<td>{{ skill.min_years }}</td>
									<td>{{ skill.max_years }}</td>
									<td class="hoverable-td">
										<router-link :to='{ name: "page-project-skill", params: { needId: need.id, skillId: skill.id } }'><i class="bi-pencil-fill p-1"></i></router-link>
										<button type='button' class='btn btn-link p-0 border-0' @click="confirmDeleteSkill(skill.id)"><i class="bi-trash-fill p-1"></i></button>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
		<router-view />
		<ConfirmDialog v-if='confirm' :title='confirm.title' @confirm='confirm.callback' @hidden-modal='confirm = null' />
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import ConfirmDialog from '../components/ConfirmDialog.vue'
	import api from '../api.js'
	
	export default {
		name: 'Project',
		data() {
			return {
				confirm: null,
			}
		},
		components: {
			VModal,
			ConfirmDialog,
		},

		computed: {
			project() {
				return this.$store.state.project
			},
		},

		methods: {
			confirmDeleteProject() {
				this.confirm = {
					title: `Delete project ${this.$store.state.project.name}`,
					callback: async () => {
						await api.deleteProject(this.$store.state.project.id)
						this.$router.push({ name: 'page-home' })
					}
				}
			},

			confirmDeleteNeed(id) {
				this.confirm = {
					title: `Delete need ${id}`,
					callback: async () => {
						await api.deleteNeed(id)
						this.$store.dispatch('setProjectNeeds', this.$route.params.id)
					}
				}
			},

			confirmDeleteSkill(id) {
				this.confirm = {
					title: `Delete skill ${id}`,
					callback: async () => {
						await api.deleteSkill(id)
						this.$store.dispatch('setProjectNeeds', this.$route.params.id)
					}
				}
			},
		},
		
		async mounted() {
			const [skills, levels] = await Promise.all([
				api.getSkills(),
				api.getLevels(),
			])

			this.$store.commit('setSkills', skills)
			this.$store.commit('setLevels', levels)

			this.$store.dispatch('setProject', this.$route.params.id)
		}
	}
</script>