<template>
	<div class="row gx-4">
		<div class="col-md-4">
			<ul class="nav nav-tabs nav-dark">
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: tabToggle }" aria-current="page" href="#" v-on:click="tabToggle = true">Find a pro</a>
				</li>
				<li class="nav-item">
					<a class="nav-link" v-bind:class="{ active: !tabToggle }" href="#" v-on:click="tabToggle = false">Find a lead</a>
				</li>
			</ul>
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<FindLead v-on:leadsfetched="passLeads" v-if="tabToggle == false" />
				<FindPro v-on:matchesfetched="passMatches" v-else />
			</div>
		</div>
		<div class="col-md-8">
			<ResultsLeads :leads='leadData' v-if="tabToggle == false" />
			<ResultsPros :matches='matchesData' v-else />
		</div>
	</div>
</template>

<script>
	import FindLead from './FindLead.vue'
	import FindPro from './FindPro.vue'
	import ResultsLeads from './ResultsLeads.vue'
	import ResultsPros from './ResultsPros.vue'
	export default {
		name: 'AdminList',
		components: {
			FindPro,
			FindLead,
			ResultsLeads,
			ResultsPros,
		},
		data() {
			return {
				tabToggle: true,
				matchesData: [],
				leadData: [],
			}
		},
		methods: {
			passMatches(value) {
				this.matchesData = value
			},
			passLeads(value) {
				this.leadData = value
			}
		}
	}
</script>