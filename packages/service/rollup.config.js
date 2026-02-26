import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { cwd } from 'node:process'
import typescript from '@rollup/plugin-typescript'

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

export default {
  input: {
    index: 'guest-js/index.ts',
    "log/index": 'guest-js/log/index.ts'
  },
  output: [
    {
      dir: 'dist',
      format: 'esm',
      entryFileNames: '[name].js',
      sourcemap: true
    },
    {
      dir: 'dist',
      format: 'cjs',
      entryFileNames: '[name].cjs',
      sourcemap: true
    }
  ],
  plugins: [
    typescript({
      declaration: true,
      declarationDir: 'dist'
    })
  ],
  external: [
    /^@tauri-apps\/api/,
    ...Object.keys(pkg.dependencies || {}),
    ...Object.keys(pkg.peerDependencies || {})
  ]
}
