import { createStore } from 'vuex'
import state from './store/state.js'
import actions from './store/actions.js'
import mutations from './store/mutations.js'
import getters from './store/getters.js'

export default createStore({
	state,
	actions,
	mutations,
	getters,
})