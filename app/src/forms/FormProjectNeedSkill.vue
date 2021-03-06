<template>
	<v-form v-on:submit="saveNeedSkill" v-if="availableSkills.length && skillLevels.length">
		<div class="mb-2" v-if="this.method == 'POST'">
			<label class="form-label">Skill</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.skill_id"
				:rules="isRequired"
				as="select"
				name="skill"
				class="form-select"
				id="AddExistingSkill"
				aria-label="Pick skill"
			>
				<option :value="null" disabled selected>Pick a skill</option>
				<option v-for="avskill in filterSkills" :key="avskill" :value="avskill.id">
					{{ avskill.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2 form-check">
			<label class="form-label">Mandatory requirement</label>
			<input type="checkbox" class="form-check-input" name="is_mandatory" v-model="formData.mandatory" />
		</div>
		<div class="mb-2">
			<label class="form-label">Minimum level</label>
			<error-message name="name" class="error"></error-message>
			<v-field
				v-model="formData.skillscopelevel_id"
				as="select"
				name="skillscope"
				class="form-select"
				id="AddLevel"
				aria-label="Pick minimum level"
			>
				<option v-for="lvl in filterLevels" :key="lvl" :value="lvl.id">
					{{ lvl.label }}
				</option>
			</v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Min years</label>
			<error-message name="min years" class="error"></error-message>
			<v-field
				v-model.number="formData.min_years"
				as="input"
				type="number"
				name="min years"
				class="form-control"
				aria-label="Min years"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Max years</label>
			<error-message name="max years" class="error"></error-message>
			<v-field
				v-model.number="formData.max_years"
				as="input"
				type="number"
				name="max years"
				class="form-control"
				aria-label="Max years"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Save</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'ProjectNeedSkill',
	data() {
		return {
			formData: {
				id: this.chosenSkill.id || undefined,
				projectneed_id: this.chosenNeed.id,
				skill_id: this.chosenSkill.skill_id || '',
				skillscopelevel_id: this.chosenSkill.skillscopelevel_id || '',
				min_years: this.chosenSkill.min_years || '',
				max_years: this.chosenSkill.max_years || '',
				mandatory: this.chosenSkill.mandatory || false,
			},
			availableSkills: {},
			skillLevels: {},
			errors: [],
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	props: {
		chosenNeed: {},
		chosenSkill: {},
		chosenScope: '',
		method: '',
		url: '',
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		async saveNeedSkill() {
			// iterate object properties and change empty values to null
			for (let prop in this.formData) {
				if (this.formData[prop] === '') {
					this.formData[prop] = null;
				}
			}

			const skill = await this.$api.needs.skills.save(this.formData)
			if (skill) this.$emit('formSent')
		},
		getSkillScope(needle) {
			if (needle) {
				var scope = this.availableSkills.find(x => x.id == needle).skillscope_id;
				this.chosenSkill.skillscope_id = scope;
			}
		},
	},
	watch: {
		'formData.skill_id'(newID) {
			this.getSkillScope(newID)
		},
		'chosenNeed.id'(newID) {
			this.formData.projectneed_id = newID
		},
	},
	computed: {
		filterLevels() {
			if (!('skillscope_id' in this.chosenSkill)) { // Projectneedskills would need to have the skillscope_id too
				this.getSkillScope(this.chosenSkill.skill_id)
			}
			return this.skillLevels.filter(lvl => lvl.skillscope_id == this.chosenSkill.skillscope_id)
		},
		filterSkills() {
			if (this.availableSkills.length) {
				if (this.chosenNeed.skills.length) {
					return this.availableSkills.filter(x => !this.chosenNeed.skills.find(y => y.skill_id == x.id));
				}
				return this.availableSkills
			}
		}
	},
	async mounted() {
		this.availableSkills = await this.$api.skills.get()
		this.skillLevels = await this.$api.skills.levels.get()
	}
};
</script>