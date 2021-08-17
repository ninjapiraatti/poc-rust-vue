export default {

	getNeed: state => id => state.project
		? state.project.needs.find(need => need.id == id) || {}
		: {},

	getSkillOfNeed: (state, getters) => (needId, skillId) =>
		getters.getNeed(needId).skills.find(skill => skill.id == skillId) || {}

}
