<template>
	<v-form v-on:submit="inviteUser">
		<div class="mb-2">
			<label class="form-label">Email</label>
			<error-message name="email" class="error"></error-message>
			<v-field
				v-model="formData.email"
				:rules="isRequired"
				as="input"
				type="text"
				name="email"
				class="form-control"
				aria-label="email"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">First name</label>
			<error-message name="first_name" class="error"></error-message>
			<v-field
				v-model="formData.first_name"
				:rules="isRequired"
				as="input"
				type="text"
				name="first_name"
				class="form-control"
				aria-label="first_name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Last name</label>
			<error-message name="last_name" class="error"></error-message>
			<v-field
				v-model="formData.last_name"
				:rules="isRequired"
				as="input"
				type="text"
				name="last_name"
				class="form-control"
				aria-label="last_name"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Password</label>
			<error-message name="password_plain" class="error"></error-message>
			<v-field
				v-model="formData.password_plain"
				:rules="isRequiredNonAdmin"
				as="input"
				type="password"
				name="password_plain"
				class="form-control"
				aria-label="password_plain"
			></v-field>
		</div>
		<div class="mb-2">
			<label class="form-label">Password again</label>
			<error-message name="password_plain_again" class="error"></error-message>
			<v-field
				v-model="formData.password_plain_again"
				:rules="doubleCheck"
				as="input"
				type="password"
				name="password_plain_again"
				class="form-control"
				aria-label="password"
			></v-field>
		</div>
		<button type="submit" class="btn btn-gradient mb-1">Register</button>
	</v-form>
</template>

<script>
import { Field, Form, ErrorMessage } from 'vee-validate';
export default {
	name: 'FormRegister',
	data() {
		return {
			user: {},
			formData: {
				email: '',
				first_name: '',
				last_name: '',
				password_plain: '',
				password_plain_again: '',
				password_pending: false,
			}
		}
	},
	components: {
		'VForm': Form,
		'VField': Field,
		ErrorMessage
	},
	methods: {
		isRequired(value) {
			return value ? true : 'This field is required';
		},
		doubleCheck(value) {
			if (this.formData.password_plain === value) {
				return true;
			} else {
				return 'Passwords do not match';
			}
		},
		isRequiredNonAdmin(value) {
			if (this.$store.state.loggeduser && this.$store.state.loggeduser.is_admin) {
				return true;
			} else if (value) {
				return true
			}
			return false
		},
		async inviteUser() {
			if (this.formData.password_plain.length == 0) {
				delete this.formData.password_plain;
				this.formData.password_pending = true;
			}

			const success = this.$api.users.registration.invite(this.formData)

			if (success) {
				this.$emit('formSent')
				this.$flashMessage.show({
					type: 'success',
					title: 'Invitation sent. Check your email',
					time: 5000,
				})
			}
		}
	}
}
</script>