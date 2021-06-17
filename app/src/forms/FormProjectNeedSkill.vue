<template>
	<form v-on:submit="createProjectNeedSkill">
		<h3>Add skill to this need</h3>
		<p v-if="errors.length && errors.includes('skill-error')" class="error">Error in adding skill. Maybe it's already added?</p>
		<label class="form-label">Skill</label>
		<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which skill" v-model="querydata_needskill.skill_id">
			<option v-for="avskill in available_skills" :key="avskill" :value="avskill.id">
				{{ avskill.label }}
			</option>
		</select>
		<label class="form-label">Minimum level</label>
		<select class="form-select mb-2" id="AddExistingSkill" aria-label="Which level" v-model="querydata_needskill.skillscopelevel_id">
			<option v-for="levelres in filterLevels" :key="levelres" :value="levelres.id">
				{{ levelres.label }}
			</option>
		</select>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</form>
</template>

<script>
export default {
	name: 'ProjectNeedSkill',
	data() {
		return {
			querydata_needskill: {
				id: '06ba4809-f20b-4687-945b-e033a6751fca',
				projectneed_id: this.chosenneed.id,
				skill_id: '',
				skillscopelevel_id: '',
				min_years: 1,
				max_years: 10,
				updated_by: this.$store.state.loggeduser.email
			},
			chosenskill: {
				type: Object,
				default() {
					return { 
						id: '',
						skillscope_id: '',
					}
				}
			},
			available_skills: {},
			skill_levels: {},
			errors: []
		}
	},
	props: {
		chosenneed: {
			type: Object,
			default() {
				return { 
					id: '',
				}
			}
		},
	},
	methods: {
		createProjectNeedSkill: function() {
			fetch('/api/projectskills', {
				method: 'POST',
				headers: {"Content-Type": "application/json"},
				credentials: 'include',
				body: JSON.stringify(this.querydata_needskill)
			})
			.catch((errors) => {
				this.errors.push('skill-error')
				console.log(errors);
			})
		},
		getAllSkills: function() {
			fetch('/api/skills', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.available_skills = response;
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		getAllLevels: function() {
			fetch('/api/skills/levels', {method: 'GET'})
			.then((response) => response.json())
			.then(response => { 
				this.skill_levels = response
			})    
			.catch((errors) => {
				console.log(errors);
			})
		},
		getSkillScope: function(needle) {
			var scope = this.available_skills.find(x => x.id == needle).skillscope_id;
			this.chosenskill.skillscope_id = scope;
		}
	},
	watch: {
		'querydata_needskill.skill_id': function(newID, oldID) {
			this.getSkillScope(newID)
		},
		'chosenneed.id': function(newID, oldID) {
			this.querydata_needskill.projectneed_id = newID
		},
	},
	computed: {
		filterLevels: function() {
			if ('skillscope_id' in this.chosenskill) {
				return this.skill_levels.filter(levelres => levelres.skillscope_id == this.chosenskill.skillscope_id)
			}
		}
	},
	mounted() {
		this.getAllSkills()
		this.getAllLevels()
	}
};
</script>