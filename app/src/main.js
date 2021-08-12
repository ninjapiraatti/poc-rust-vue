import { createApp } from 'vue'
import { createStore } from 'vuex'
import FlashMessage from '@smartweb/vue-flash-message';
import setupRouter from './router.js'
import App from './App.vue'
import 'bootstrap'
import { Modal } from 'bootstrap'


// Create a new store instance.

const store = createStore({
	state () {
		return {
			loggeduser: JSON.parse(localStorage.getItem('user')),
			chosenproject: {},
			projects: JSON.parse(localStorage.getItem('projects')),
			nextpage: '',
			errorObject: null,
		}
	},
	actions: {
		async setUser(context, data) {
			const userData = await fetch(`/api/users/${data}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then((response) => response.json())
			context.commit('setUser', userData)
		},
		async setChosenProject (context, data) {
			try {
				let hasNeeds = true
				let project = await fetch(`/api/projects/${data}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.then((response) => response.json())
				project.needs = await fetch(`/api/projectneeds/${data}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.then((response) => response.json())
				.catch((errors) => {
					//console.log(errors)
					project.needs = {}
					hasNeeds = false
				})
				if (hasNeeds === true) {
					await Promise.all(project.needs.map(need =>
						fetch(`/api/projectskills/${need.id}`, {
							method: 'GET',
							headers: {"Content-Type": "application/json"},
							credentials: 'include'
						})
						.then((response) => response.json())
						.then(response => {
							need.skills = response
						})
						.catch((errors) => {
							//console.log(errors)
							need.skills = {}
						})
					))
				}
				context.commit('setChosenProject', project)
			} catch (errors) {
				console.log(errors)
			}
		},
	},
	mutations: {
		setUser(state, data) {
			state.loggeduser = data;
			localStorage.setItem('user', JSON.stringify(data));
		},
		setChosenProject(state, data) {
			state.chosenproject = data
		},
		getProjects (state) {
			let projectsExist = true
			let projects = []
			fetch('/api/projects?is_include_skills_and_matches=true', {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then(response => {
				if (response.status == 204) {
					projects = []
					projectsExist = false
				} else {
					return response.json()
				}
			})
			.catch((errors) => {
				console.log(errors);
			})
			.then(response => {
				if (projectsExist === true) {
					projects = response
					projects.forEach(function (project) {
						fetch(`/api/projectneeds/${project.id}`, {
							method: 'GET',
							headers: {"Content-Type": "application/json"},
							credentials: 'include'
						})
						.then((response) => response.json())
						.catch((errors) => {
							console.log(errors)
							project.needs = {}
						})
						.then((response) => {
							project.needs = response
						})
					});
					localStorage.setItem('projects', JSON.stringify(projects));
					state.projects = projects
				} else {
					projects = []
					localStorage.setItem('projects', JSON.stringify(projects));
					state.projects = projects
				}
			})
		},
		resetChosenProject (state) {
			state.chosenproject = {}
		},
		errorHandling (state, error) {
			if(error.status == 401) {
				state.loggeduser = null
				localStorage.removeItem('user');
				router.push({ name: 'page-login' })
				return "Unauthorized"
			}
			if(error.status == 500) {
				state.errorObject = {
					type: 'error',
					title: 'Error 500',
					time: 1000
				}
				router.push({ name: 'page-error' })
				return "Unauthorized"
			}
			if(error.status == 400) {
				state.errorObject = {
					type: 'error',
					title: 'Error 400',
					time: 1000
				}
			}
			let errorObject = Promise.resolve(error)
			errorObject.then((resError) => resError.json())
			.then(errObject => {
				switch (errObject.error_type) {
					case 'UniqueViolation':
						state.errorObject = {
							type: 'error',
							title: 'Item already exists.',
							time: 5000
						}
						break;
					case 'AdminRequired':
						state.loggeduser = null
						localStorage.removeItem('user');
						router.push({ name: 'page-login' })
						break;
					case 'ForeignKeyViolation':
						state.errorObject = {
							type: 'error',
							title: 'Foreign key violation',
							time: 5000
						}
						break;
					default:
						console.log(errObject)
				}
			})
		},
		deleteUser (state) {
			state.loggeduser = null
			localStorage.removeItem('user');
			router.push({ name: 'page-login' })
		}
	}
})

const router = setupRouter()
const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)

// Close modal before navigating
router.beforeEach((to, from, next) => {
	const modal = document.querySelector('.show[id^=hulaModal]')
	if (!modal) {
		next()
	} else {
		modal.addEventListener('hidden.bs.modal', () => next())
		Modal.getInstance(modal).hide()
	}
})

router.beforeEach((to, from, next) => {
	console.log(to)
	if (to.name !== 'page-login' 
		&& to.name !== 'page-confirm'
		&& to.name !== 'page-forgot-password'
		&& !store.state.loggeduser) {
			store.state.nextpage = to.fullPath
			next({ name: 'page-login' })
	}
	else next()
})
router.isReady()
	.then(() => app.mount('#hula'))
