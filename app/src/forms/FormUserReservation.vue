<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div>
			<label for='description' class='form-label'>Description</label>
			<VField
				v-model='form.description'
				rules='required'
				type='text'
				id='description'
				name='description'
				label='Description'
				aria-label='Description'
				class='form-control'
				:class='{ "is-invalid": errors.description }'
			/>
			<ErrorMessage name='description' class='invalid-feedback shake' />
		</div>

		<div>
			<label class='form-label'>Start date</label>
			<VField
				v-model='form.begin_time'
				rules='required|date'
				type='date'
				id='begin_time'
				name='begin_time'
				label='Start date'
				aria-label='Start date'
				class='form-control'
				:class='{ "is-invalid": errors.begin_time }'
			/>
			<ErrorMessage name='begin_time' class='invalid-feedback shake' />
		</div>

		<div>
			<label class='form-label'>End date</label>
			<VField
				v-model='form.end_time'
				rules='date|afterDate:begin_time'
				type='date'
				id='end_time'
				name='end_time'
				label='End date'
				aria-label='End date'
				class='form-control'
				:class='{ "is-invalid": errors.end_time }'
			/>
			<ErrorMessage name='end_time' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='percentage' class='form-label'>Percentage</label>
			<VField
				v-model.number='form.percentage'
				rules='required'
				type='number'
				name='percentage'
				id='percentage'
				label='Percentage'
				aria-label='Percentage'
				class='form-control'
				:class='{ "is-invalid": errors.percentage }'
			/>
			<ErrorMessage name='percentage' class='invalid-feedback shake' />
		</div>

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm>
</template>

<script>
	export default {
		name: 'FormUserReservation',

		props: {
			id: {
				type: String,
				default: undefined,
			},
			user_id: {
				type: String,
				required: true,
			},
			description: String,
			begin_time: Date,
			end_time: Date,
			percentage: Number,
		},

		data() {
			const form = { ...this.$props }

			for (const prop in form) {
				if (form[prop] instanceof Date) form[prop] = [
					form[prop].getFullYear(),
					form[prop].getMonth() + 1,
					form[prop].getDate(),
				].map(nr => String(nr).padStart(2, 0)).join('-')
			}

			return {
				sending: false,
				form,
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

				for (const prop in this.form) if (this.form[prop] == '') this.form[prop] = undefined
				const reservation = await this.$api.users.reservations.save(this.form)
				if (reservation) this.$emit('success', reservation)

				this.sending = false
			},
		},
	}
</script>
