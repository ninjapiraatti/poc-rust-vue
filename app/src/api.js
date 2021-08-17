import state from './store/state.js'

export default {
	async getProject(id) {
		return fetch(`/api/projects/${id}`, {
			method: 'GET',
			headers: { "Content-Type": "application/json" },
			credentials: 'include',
		})
			.then(response => response.json())
			.catch(() => null)
	},

	async getProjectNeeds(id) {
		const needs = await fetch(`/api/projectneeds/${id}`, {
			method: 'GET',
			headers: { "Content-Type": "application/json" },
			credentials: 'include',
		})
			.then(response => response.json())
			.catch(() => [])
		
		await Promise.all(needs.map(need =>
			this.getSkillsForNeed(need.id)
				.then(skills => {
					need.skills = skills.map(skill => {
						skill.label = state.skills.find(({ id }) => skill.skill_id == id).label
						skill.level = (state.levels.find(({ id }) => skill.skillscopelevel_id == id) || {}).label
						return skill
					})
				})
		))

		return needs
	},

	async getSkills() {
		return fetch('/api/skills', { method: 'GET' })
			.then(response => response.json())
			.catch(() => [])
	},

	async getLevels() {
		return fetch('/api/skills/levels', { method: 'GET' })
			.then(response => response.json())
			.catch(() => [])
	},

	async getSkillsForNeed(id) {
		return fetch(`/api/projectskills/${id}`, { method: 'GET' })
			.then((response) => response.json())
			.catch(() => [])
	},

	async deleteProject(id) {
		return fetch(`/api/projects/${id}`, { method: 'DELETE' })
	},
	
	async deleteNeed(id) {
		return fetch(`/api/projectneeds/${id}`, { method: 'DELETE' })
	},

	async deleteSkill(id) {
		return fetch(`/api/projectskills/${id}`, { method: 'DELETE' })
	},
}