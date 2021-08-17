<template>
	<VModal :instant='true' modalTitle='Edit project' modalID='ProjectEdit' @hidden-modal='hideModal'>
		<Form @submit='submit'>
			<div class='mb-2'>
				<label class='form-label'>Project name</label>
				<error-message name='name' class='error'></error-message>
				<Field
					v-model='formData.name'
					rules='required'
					as='input'
					type='text'
					name='name'
					class='form-control'
					aria-label='Project name'
				></Field>
			</div>
			<div class='mb-2 form-check'>
				<label class='form-label'>Hidden</label>
				<error-message name='category' class='error'></error-message>
				<input type='checkbox' class='form-check-input' name='is_hidden' v-model='formData.is_hidden' />
			</div>
			<button type='submit' class='btn btn-gradient mb-1'>Submit</button>
		</Form> 
	</VModal>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import { Field, Form, ErrorMessage } from 'vee-validate'

	export default {
		name: 'ProjectEdit',

		components: {
			Form,
			Field,
			ErrorMessage,
			VModal,
		},

		data() {
			return {
				formData: {
					name: this.$store.state.project.name || '',
					is_hidden: this.$store.state.project.is_hidden || false, // TODO: Does this work in both edit and new?
				},
			}
		},

		methods: {
			hideModal() {
				this.$router.replace({ name: 'page-project' })
			},

			async submit() {
				const method = this.$store.state.project ? 'PUT' : 'POST'

				let url = '/api/projects'
				if (this.$store.state.project) url+= `/${this.$store.state.project.id}`

				const data = { ...this.formData }

				await fetch(url, {
					method,
					headers: {'Content-Type': 'application/json'},
					credentials: 'include',
					body: JSON.stringify(data)
				})
				
				this.$store.dispatch('setProject', this.$store.state.project.id)
				this.hideModal()
			},
		},
	}
</script>