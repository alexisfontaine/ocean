module.exports = {
	plugins: {
		autoprefixer: {},
		cssnano: { preset: 'advanced' },
		'postcss-combine-duplicated-selectors': { removeDuplicatedProperties: true },
	}
}
