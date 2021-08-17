<template>
	<VModal :instant='true' :modalTitle='title' modalID='Skill' @hidden-modal='hideModal'>
		<Form @submit="submit">
			<div class="mb-2" v-if="!this.$route.params.skillId">
				<label class="form-label">Skill</label>
				<error-message name="name" class="error"></error-message>
				<Field
					v-model="formData.skill_id"
					rules="required"
					as="select"
					name="skill"
					class="form-select"
					id="AddExistingSkill"
					aria-label="Pick skill"
				>
					<option :value="null" disabled selected>Pick a skill</option>
					<option v-for="skill in availableSkills" :key="skill.id" :value="skill.id">{{ skill.label }}</option>
				</Field>
			</div>
			<div class="mb-2 form-check">
				<label class="form-label">Mandatory requirement</label>
				<input type="checkbox" class="form-check-input" name="is_mandatory" v-model="formData.mandatory" />
			</div>
			<div class="mb-2">
				<label class="form-label">Minimum level</label>
				<error-message name="name" class="error"></error-message>
				<Field
					v-model="formData.skillscopelevel_id"
					as="select"
					name="skillscope"
					class="form-select"
					id="AddLevel"
					aria-label="Pick minimum level"
				>
					<option v-for="level in availableLevels" :key="level.id" :value="level.id">{{ level.label }}</option>
				</Field>
			</div>
			<div class="mb-2">
				<label class="form-label">Min years</label>
				<error-message name="min years" class="error"></error-message>
				<Field
					v-model.number="formData.min_years"
					as="input"
					type="number"
					name="min years"
					class="form-control"
					aria-label="Min years"
				></Field>
			</div>
			<div class="mb-2">
				<label class="form-label">Max years</label>
				<error-message name="max years" class="error"></error-message>
				<Field
					v-model.number="formData.max_years"
					as="input"
					type="number"
					name="max years"
					class="form-control"
					aria-label="Max years"
				></Field>
			</div>
			<button type="submit" class="btn btn-gradient mb-1">Save</button>
		</Form>
	</VModal>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Field, Form, ErrorMessage } from 'vee-validate'

	export default {
		name: 'ProjectNeedSkill',

		components: {
			Form,
			Field,
			ErrorMessage,
			VModal,
		},

		data() {
			const skill = this.$store.getters.getSkillOfNeed(this.$route.params.needId, this.$route.params.skillId)
			
			return {
				title: skill.label || 'Add skill',
				formData: {
					id: this.$route.params.skillId || undefined,
					projectneed_id: this.$route.params.needId,
					skill_id: skill.skill_id || undefined,
					skillscopelevel_id: skill.skillscopelevel_id || undefined,
					min_years: skill.min_years || undefined,
					max_years: skill.max_years || undefined,
					mandatory: skill.mandatory || false,
				},
			}
		},
		
		methods: {
			hideModal() {
				this.$router.replace({ name: "page-project" })
			},

			async submit() {
				const method = this.$route.params.skillId ? 'PUT' : 'POST'
				let url = '/api/projectskills'
				if (this.$route.params.skillId) url+= `/${this.$route.params.skillId}`
				
				const data = { ...this.formData }

				await fetch(url, {
					method,
					headers: {"Content-Type": "application/json"},
					credentials: 'include',
					body: JSON.stringify(data)
				})

				this.$store.dispatch('setProjectNeeds', this.$route.params.id)
				this.hideModal()
			},
		},
		
		computed: {
			need() {
				return this.$store.getters.getNeed(this.$route.params.needId)
			},

			scope() {
				const skill = this.$store.state.skills.find(skill => skill.id == this.formData.skill_id)
				return skill ? skill.skillscope_id : null
			},

			availableSkills() {
				if (this.need.skills.length) {
					return this.$store.state.skills.filter(({ id }) => !this.need.skills.find(skill => skill.skill_id == id))
				}
				return this.$store.state.skills
			},

			availableLevels() {
				return this.$store.state.levels.filter(level => level.skillscope_id == this.scope)
			},
		},

	}
</script>