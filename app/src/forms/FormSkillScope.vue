<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<label for='label' class='form-label'>Name</label>
		<div class='input-group' :class='{ "has-validation": errors.label }'>
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				id='label'
				name='label'
				label='Name'
				aria-label='Name'
				placeholder='One to ten' 
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient'>{{ submitLabel }}</button>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

	</VForm>  
</template>

<script>
	export default {
		name: 'FormSkillScope',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			label: String,
		},	

		data() {
			return {
				sending: false,
				form: { ...this.$props },
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Saving' : 'Save'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const scope = await this.$api.skills.scopes.save(this.form)
				if (scope) this.$emit('success', scope)

				this.sending = false
			}
		},
	}
</script>
