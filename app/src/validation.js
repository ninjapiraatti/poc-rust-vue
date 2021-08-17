import { defineRule } from 'vee-validate'

defineRule('required', value => {
	return value ? true : 'This field is required'
})

defineRule('requiredDate', value => {
	const date = new Date(value)
	return date.toString() === 'Invalid Date' ? 'This field is required' : true
})

defineRule('afterStartDate', (value, [start]) => {
	start = new Date(start)
	const end = new Date(value)
	return start <= end ? true : 'End time must be after start time.'
})