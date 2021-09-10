<template>
	<div class="container">
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='card-header'>
						<h1 class="h3 mb-0">{{ user.firstname }} {{ user.lastname }}</h1>
					</div>
					<div class='card-body'>
						<p>{{ user.email }}</p>
						<button class='btn btn-unstyled' v-on:click="editUser(user)"><i class="bi-pencil-fill me-2"></i></button>
						<button class='btn btn-unstyled' v-on:click="confirmDelete('user', user)"><i class="bi-trash-fill me-2"></i></button>
						<hr />
						<VForm @submit="saveFiles" class='clearfix'>
							<div class="mb-3">
								<table v-if='files.length' class="table table-striped" :class='$colorScheme.table'>
									<thead>
										<tr>
											<th scope="col">CV</th>
											<th scope="col" colspan='2'>File</th>
										</tr>
									</thead>
									<tbody>
										<tr v-for="file in files" :key="file.id">
											<td><input type='checkbox' :checked='user.main_upload_id == file.id' @click='setCV(file.id)'></td>
											<td><a href='#' @click.prevent>{{ file.filename }}</a></td>
											<td><button class='btn btn-unstyled' v-on:click="confirmDelete('user.file', file)"><i class="bi-trash-fill me-2"></i></button></td>
										</tr>
									</tbody>
								</table>
								<label class="form-label">Upload files</label>
								<VField
									type="file"
									name="newFiles[]"
									multiple
									class="form-control" 
									v-model="newFiles"
								/>
							</div>
							<button type="submit" class="btn btn-primary gradient float-end">Upload files</button>
						</VForm>
					</div>
				</div>
			</div>
			<div class="col-md-8">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='card-header'>
						<div class="d-sm-flex flex-row justify-content-between align-items-center">
							<h3 class="h3 mb-0">Skills</h3>
							<button class="btn btn-primary gradient" v-on:click="editSkill()">Add skill</button>
						</div>
					</div>
					<div class='card-body'>
						<div class="table-responsive">
							<table class="table table-striped" :class='$colorScheme.table'>
								<thead>
									<tr>
										<th scope="col">Skill</th>
										<th scope="col" class='text-center'>Level</th>
										<th scope="col" class='text-center'>Years</th>
										<th scope="col" class='text-end'>Actions</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="skill in user.skills" :key="skill.id">
										<td>{{ skill.skill_label }}</td>
										<td class='text-center'>{{ skill.levelLabel }}</td>
										<td class='text-center'>{{ skill.years }}</td>
										<td class='text-end'>
											<button class='btn btn-unstyled' v-on:click="editSkill(skill)"><i class="bi-pencil-fill me-2"></i></button>
											<button class='btn btn-unstyled' v-on:click="confirmDelete('user.skill', skill)"><i class="bi-trash-fill me-2"></i></button>
										</td>
									</tr>
								</tbody>
							</table>
						</div>
					</div>
				</div>
				<div class="card shadow mt-4" :class='$colorScheme.card'>
					<div class='card-header'>
						<div class="d-sm-flex flex-row justify-content-between align-items-center">
							<h3 class="h3 mb-0">Reservations</h3>
							<button class="btn btn-primary gradient" v-on:click="editReservation()">Add reservation</button>
						</div>
					</div>
					<div class='card-body'>
						<table class="table table-striped" :class='$colorScheme.table' v-if="reservations.length">
							<thead>
								<tr>
									<th scope="col">Description</th>
									<th scope="col">From</th>
									<th scope="col">To</th>
									<th scope="col" class='text-center'>Percentage</th>
									<th scope="col" class='text-end'>Actions</th>
								</tr>
							</thead>
							<tbody>
								<tr v-for="reservation in reservations" :key="reservation.id">
									<td>{{ reservation.description }}</td>
									<td>{{ reservation.begin_time }}</td>
									<td>{{ reservation.end_time }}</td>
									<td class='text-center'>{{ reservation.percentage }}</td>
									<td class='text-end'>
										<button class='btn btn-unstyled' v-on:click="editReservation(reservation)"><i class="bi-pencil-fill me-2"></i></button>
										<button class='btn btn-unstyled' v-on:click="confirmDelete('user.reservation', reservation)"><i class="bi-trash-fill me-2"></i></button>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
				<div v-if='matches.length' class="card shadow mt-4" :class='$colorScheme.card'>
					<div class='card-header'>
						<h3 class="h3 mb-0">Projects matching the user's skills</h3>
					</div>
					<div class='card-body'>
						<VMatchesForUser :matches='matches' />
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import FormUserInfo from '../forms/FormUserInfo.vue'
	import FormUserSkill from '../forms/FormUserSkill.vue'
	import FormUserReservation from '../forms/FormUserReservation.vue'
	import VMatchesForUser from '../components/VMatchesForUser.vue'

	export default {
		name: 'UserProfile',

		components: {
			VMatchesForUser,
		},

		data() {
			return {
				user: {},
				reservations: [],
				files: [],
				newFiles: [],
			}
		},

		computed: {
			matches() {
				return this.$store.state.projects.filter(project => {
					return project.matches.some(match =>
						match.user_id == this.user.id
					)
				})			
			},
		},

		async mounted() {
			if (this.$store.state.loggeduser.isadmin && !this.$store.state.projects.length) {
				this.$store.dispatch('getProjects')
			}

			await Promise.all([
				this.getUser(),
				this.getReservations(),
				this.getUploads(),
			])
		},

		methods: {
			async getUser() {
				const promises = [ this.$api.users.get(this.$route.params.id) ]
				if (!this.$store.state.skillLevels.length) promises.push(this.$store.dispatch('getSkillLevels'))

				const [ user ] = await Promise.all(promises)

				this.user = user

				this.user.skills.forEach(skill => {
					skill.levelLabel = this.$store.state.skillLevels.find(({ id }) => id == skill.skillscopelevel_id).label
				})
			},

			async getReservations() {
				this.reservations = await this.$api.users.reservations.get(this.$route.params.id)
			},

			async getUploads() {
				const files = await this.$api.users.files.get(this.$route.params.id)
				this.files = files
			},

			async saveFiles() {
				const success = await this.$api.users.files.save(this.newFiles)

				const message = success ? {
					type: 'success',
					title: 'Upload complete',
					text: `${this.newFiles.length} file${this.newFiles.length > 1 ? 's were' : ' was'} uploaded successfully`,
				} : {
					type: 'error',
					title: 'Upload failed',
				}

				this.$flashMessage.show({
					...message,
					time: 5000,
				})

				this.newFiles = []

				if (success) this.getUploads()
			},

			async setCV(id) {
				const value = this.user.main_upload_id == id ? null : id

				const data = await this.$api.users.save({
					...this.user,
					main_upload_id: value,
				})

				if (data) this.user.main_upload_id = value
			},

			async editUser(props = {}) {
				const result = await this.$modal({
					title: 'Edit user info',
					component: FormUserInfo,
					props,
				})

				if (result) this.getUser()
			},

			async editSkill(props = {}) {
				props.user_id = this.user.id

				const result = await this.$modal({
					title: props.id ? `Edit skill: ${props.skill_label}` : 'Add skill',
					component: FormUserSkill,
					props,
				})

				if (result) this.getUser()
			},

			async editReservation(props = {}) {
				props.user_id = this.user.id

				const result = await this.$modal({
					title: props.id ? `Edit reservation: ${props.description}` : 'Add reservation',
					component: FormUserReservation,
					props,
				})

				if (result) this.getReservations()
			},

			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				
				if (success) {
					switch (type) {
						case 'user':
							if (data.id == this.$store.state.loggeduser.id) await this.$api.users.log.out()
							this.$router.push({ name: 'admin-users' })
							break

						case 'user.skill':
							this.getUser()
							break
						
						case 'user.reservation':
							this.getReservations()
							break
						
						case 'user.file':
							this.getUploads()
							break
					}
				}
			}
		},
	}
</script>