import { createRouter, createWebHistory } from 'vue-router'
import * as views from './views.js'

const setupRoute = entry => ({
	path: entry.path,
	name: `${entry.id}`,
	component: views[entry.component],
	meta: {
		id: entry.id,
		title: entry.title,
		hidden: entry.hidden,
	},
})

export default function setupRouter() {
	const router = createRouter({
		routes: [
			{ path: '/:pathMatch(.*)*', component: views.Page404 },
			{ path: '/', component: views.Home },
		],
		history: createWebHistory(),
		linkActiveClass: 'active',
		linkExactActiveClass: ''
	})
	return router
}
