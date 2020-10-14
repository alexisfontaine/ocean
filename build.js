const { execSync } = require('child_process')
const { join } = require('path')

const Bundler = require('parcel-bundler')
const chokidar = require('chokidar')
const project = require('./package')

const IS_PRODUCTION = process.env.NODE_ENV === 'production'
const NAME = project.name
const OUTPUT = join(__dirname, 'target', IS_PRODUCTION ? 'release' : 'debug')
const PACKAGE = join(OUTPUT, 'package')
const PUBLIC = join(OUTPUT, 'public')

const bundler = new Bundler(join(__dirname, 'index.html'), {
	autoInstall: false,
	cacheDir: join(OUTPUT, '.cache'),
	outDir: PUBLIC,
	scopeHoist: IS_PRODUCTION,
})

const build = `PACKAGE_DIR=${PACKAGE} wasm-pack build \
	--no-typescript \
	--out-dir ${PACKAGE} \
	--out-name ${NAME} \
	--target web \
	${IS_PRODUCTION ? '-- --features wee_alloc' : '--dev'}`

const copy = `cp -r ${join(PACKAGE, `{${NAME}.js,${NAME}_bg.wasm}`)} ${join(__dirname, 'assets/{images,favicon.ico,logo.svg,manifest.webmanifest}')} ${PUBLIC}`

bundler.on('buildStart', () => execSync(build, { stdio: 'inherit' }))
bundler.on('buildEnd', () => execSync(copy))

if (IS_PRODUCTION) {
	execSync(`rm -rf ${PUBLIC}`)
	bundler.bundle()
} else {
	chokidar
		.watch(['./{sources,components}/**/*.{rs,toml}', './Cargo.toml'])
		.on('change', async (event, path) => {
			console.log(`"${path}" changed...`)

			bundler.bundle()
			bundler.hmr.broadcast({ type: 'reload' })
		})

	bundler.serve(process.env.PORT || 1234)
}
