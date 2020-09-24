const { execSync } = require('child_process')
const { join } = require('path')

const Bundler = require('parcel-bundler')
const chokidar = require('chokidar')
const project = require('./package')

const isProduction = process.env.NODE_ENV === 'production'
const output = join(__dirname, 'target', isProduction ? 'release' : 'debug')

const command = `wasm-pack build --no-typescript --target web --out-dir ${join(output, 'public')} --out-name ${project.name} ${isProduction ? '-- --features wee_alloc' : '--dev'}`

;(async () => {
	const bundler = new Bundler(join(__dirname, 'index.html'), {
		cacheDir: join(output, '.cache'),
		outDir: join(output, 'public'),
		outFile: 'index.html',
		publicUrl: '/',
		watch: !isProduction,
		minify: isProduction,
	})

	bundler.on('buildStart', () => execSync(command, { stdio: 'inherit' }))

	if (isProduction) {
		execSync(`rm -rf ${join(output, 'public')}`)
		bundler.bundle()

		return
	}

	chokidar
		.watch(['./{sources,components}/**/*.{rs,toml}', './Cargo.toml'])
		.on('change', async (event, path) => {
			console.log(`"${path}" changed...`)

			bundler.bundle()
			bundler.hmr.broadcast({ type: 'reload' })
		})

	await bundler.serve(process.env.PORT || 1234)
})()
