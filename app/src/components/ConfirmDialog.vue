<template>
	<VModal :instant='true' ref='modal' :modalTitle='title' modalID='Confirm'>
		<div v-if='!!this.$slots.default' class='mb-3'>
			<slot />
		</div>
		<div>
			<button type="button" class="btn btn-gradient mb-1" @click='confirm'>Confirm</button>
		</div>
	</VModal>
</template>

<script>
	import VModal from './VModal.vue'

	export default {
		name: 'ConfirmAction',

		components: {
			VModal,
		},

		props: {
			title: String,
		},

		methods: {
			show() {
				this.$refs.modal.show()
			},
			
			confirm() {
				this.$emit('confirm')
				this.$refs.modal.hide()
			},
		},
	}
</script>