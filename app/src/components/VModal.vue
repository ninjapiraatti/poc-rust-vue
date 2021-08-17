<template>
	<div class="modal fade" ref="hulaModal" :id="'hulaModal' + modalID" :data-bs-backdrop="modalStatic ? 'static' : null">
		<div class="modal-dialog">
			<div class="modal-content rounded-2 content-box bg-dark text-light">
				<div class="modal-header" v-if="modalStatic == false">
					<h2 class="modal-title">{{ modalTitle }}</h2>
					<button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal" aria-label="Close"></button>
				</div>
				<div class="modal-body">
					<slot></slot>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import { Modal } from 'bootstrap'

export default {
	name: 'VModal',
	props: {	
		instant: false,
		modalTitle: String,
		modalID: String,
		modalStatic: {
			type: Boolean,
			default: false
		}
	},

	mounted() {
		this.modal = Modal.getOrCreateInstance(this.$refs.hulaModal)

		if (this.instant) this.modal.show()

		this.$refs.hulaModal.addEventListener('show.bs.modal', () => { this.$emit('showingModal') })
		this.$refs.hulaModal.addEventListener('shown.bs.modal', () => { this.$emit('showModal') })
		this.$refs.hulaModal.addEventListener('hidden.bs.modal', () => { this.$emit('hideModal') })
		this.$refs.hulaModal.addEventListener('hidden.bs.modal', () => { this.$emit('hiddenModal') })
	},

	updated() {
		this.$refs.hulaModal.addEventListener('hidden.bs.modal', () => {
			this.$emit('updatedModal')
		})
	},

	methods: {
		show() {
			this.modal.show()
		},

		hide() {
			this.modal.hide()
		},
	},
};
</script>