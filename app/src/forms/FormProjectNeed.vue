<template>
	<VModal :instant='true' :modalTitle='title' modalID='Need' @hidden-modal='hideModal'>
		<Form @submit="submit">
			<h2 v-if="$route.params.needId">{{ $route.params.needId }}</h2>
			<h2 v-else>New need</h2>
			<div class="mb-2">
				<label class="form-label">How many pros for this need?</label>
				<error-message name="users" class="error"></error-message>
				<Field
					v-model.number="formData.count_of_users"
					rules="required"
					as="input"
					type="number"
					name="users"
					class="form-control"
					aria-label="Number of pros" 
				></Field>
			</div>
			<div class="mb-2">
				<label class="form-label">When does this need start?</label>
				{{ formData.begin_time }}<br />
				<error-message name="begintime" class="error"></error-message>
				<Field
					v-model="formData.begin_time"
					rules="requiredDate"
					as="input"
					type="date"
					name="begintime"
					class="form-control"
					aria-label="Date start" 
				></Field>
			</div>
			<div class="mb-2">
				<label class="form-label">When does this need end?</label>
				{{ formData.end_time }}<br />
				<error-message name="endtime" class="error"></error-message>
				<Field
					v-model="formData.end_time"
					:rules='`required|afterStartDate:${formData.begin_time}`'
					as="input"
					type="date"
					name="endtime"
					class="form-control"
					aria-label="Date end" 
				></Field>
			</div>
			<div class="mb-2">
				<label class="form-label">At what percentage?</label>
				<error-message name="percentage" class="error"></error-message>
				<Field
					v-model.number="formData.percentage"
					rules="required"
					as="input"
					type="number"
					name="percentage"
					class="form-control"
					aria-label="Percentage" 
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
		name: "ProjectNeed",

		components: {
			Form,
			Field,
			ErrorMessage,
			VModal,
		},

		data() {
			const data = this.$store.getters.getNeed(this.$route.params.needId)
			
			return {
				title: this.$route.params.needId ? 'Edit need' : 'Add need',
				formData: {
					project_id: this.$route.params.id,
					count_of_users: data.count_of_users || undefined,
					begin_time: data.begin_time || undefined,
					end_time: data.end_time || undefined,
					percentage: data.percentage || undefined,
				}
			}
		},

		methods: {
			hideModal() {
				this.$router.replace({ name: "page-project" })
			},

			async submit() {
				const method = this.$route.params.needId ? 'PUT' : 'POST'
				
				let url = '/api/projectneeds'
				if (this.$route.params.needId) url+= `/${this.$route.params.needId}`
				
				const data = { ...this.formData }

				if (!data.end_time) delete data.end_time
				
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
	}
</script>