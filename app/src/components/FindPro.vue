<template>
	<div>
		<h2 v-if="formData.projectname.length">{{ formData.projectname }}</h2>
		<VAutoComplete 
			v-if="this.$store.state.projects" 
			:suggestions="this.$store.state.projects"
			:placeholder="'Start typing the name of project..'" 
			:dropdown="true"
			:filterProperties="'name'"
			:selection="value"
			v-on:auto-complete="getMatches"
		></VAutoComplete>
	</div>
</template>

<script>
	import VAutoComplete from '../components/VAutoComplete.vue'
	export default {
		name: 'FindPro',
		data() {
			return {
				value: '',
				projects: this.$store.state.projects,
				selected: {},
				formData: {
					projectname: '',
				},
			}
		},
		components: {
			VAutoComplete
		},
		mounted() {
			if (!this.$store.state.projects.length) this.$store.dispatch('getProjects')	
		},
		methods: {
			async getMatches(project) {
				const matches = await this.$api.matches.get(project.id)
				if (matches) this.$emit('matchesfetched', matches)
			},
		}
	}
</script>
