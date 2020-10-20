module.exports = {
	includePaths: [`target/${process.env.NODE_ENV === 'production' ? 'release' : 'debug'}/package`]
}
