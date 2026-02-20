#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');

// 获取平台和架构信息
const platform = os.platform();
const arch = os.arch();

console.log(`Platform: ${platform}, Architecture: ${arch}`);

// 平台映射表
const platformMap = {
  'win32': 'windows',
  'darwin': 'macos',
  'linux': 'linux',
  'freebsd': 'freebsd',
  'openbsd': 'openbsd',
  'sunos': 'solaris',
  'android': 'android'
};

// 架构映射表
const archMap = {
  'x64': 'x86_64',
  'arm64': 'aarch64',
  'arm': 'armv7',
  'ia32': 'i686',
  'mips': 'mips',
  'mipsel': 'mipsel',
  'ppc': 'powerpc',
  'ppc64': 'powerpc64'
};

// 获取可执行文件名（支持跨平台命名）
function getExecutableName() {
  const baseName = 'winfox';
  const platformName = platformMap[platform] || platform;
  const archName = archMap[arch] || arch;

  switch (platform) {
    case 'win32':
      return `${baseName}.exe`;
    case 'darwin': // macOS
      // macOS 通常不需要扩展名
      return baseName;
    case 'linux':
    case 'freebsd':
    case 'openbsd':
    case 'sunos':
    case 'android':
    default:
      // 对于其他平台，可以包含平台和架构信息
      return `${baseName}-${platformName}-${archName}`;
  }
}

// 查找可执行文件（处理不同的构建输出）
function findExecutableFile(sourceDir) {
  if (!fs.existsSync(sourceDir)) {
    return null;
  }

  const files = fs.readdirSync(sourceDir);
  const expectedName = getExecutableName();

  console.log(`Looking for executable: ${expectedName}`);

  // 1. 首先查找预期的文件名
  if (files.includes(expectedName)) {
    return expectedName;
  }

  // 2. 查找简单的 winfox 或 winfox.exe
  const simpleNames = platform === 'win32' ? ['winfox.exe'] : ['winfox'];
  for (const name of simpleNames) {
    if (files.includes(name)) {
      return name;
    }
  }

  // 3. 查找任何以 winfox 开头的文件
  const winfoxFiles = files.filter(file =>
    file.startsWith('winfox') ||
    file.startsWith('winterfox-cli')
  );

  if (winfoxFiles.length > 0) {
    // 返回第一个匹配的文件
    return winfoxFiles[0];
  }

  // 4. 查找任何可执行文件（Unix-like 平台）
  if (platform !== 'win32') {
    const executableFiles = files.filter(file => {
      try {
        const filePath = path.join(sourceDir, file);
        const stats = fs.statSync(filePath);
        // 检查是否是可执行文件
        return stats.isFile() && (stats.mode & 0o111) !== 0;
      } catch {
        return false;
      }
    });

    if (executableFiles.length > 0) {
      return executableFiles[0];
    }
  }

  return null;
}

// 主函数
function main() {
  // 确定源目录和目标目录
  const sourceDir = path.join(__dirname, '../../../target/release');
  const destDir = path.join(__dirname, '../bin');

  console.log(`Source directory: ${sourceDir}`);
  console.log(`Destination directory: ${destDir}`);

  // 查找可执行文件
  const executableName = findExecutableFile(sourceDir);

  if (!executableName) {
    console.error('❌ Error: No executable file found in target/release/');
    console.error('Please run `cargo build --release` first.');

    // 列出可用的文件
    if (fs.existsSync(sourceDir)) {
      console.log('\nAvailable files in target/release/:');
      const files = fs.readdirSync(sourceDir);
      if (files.length === 0) {
        console.log('  (empty directory)');
      } else {
        files.forEach(file => {
          const filePath = path.join(sourceDir, file);
          try {
            const stats = fs.statSync(filePath);
            const size = (stats.size / 1024).toFixed(2);
            const isExecutable = (stats.mode & 0o111) !== 0;
            console.log(`  - ${file} (${size} KB${isExecutable ? ', executable' : ''})`);
          } catch {
            console.log(`  - ${file}`);
          }
        });
      }
    }

    process.exit(1);
  }

  const sourceFile = path.join(sourceDir, executableName);
  const destFile = path.join(destDir, executableName);

  console.log(`Source file: ${sourceFile}`);
  console.log(`Destination file: ${destFile}`);

  // 确保目标目录存在
  if (!fs.existsSync(destDir)) {
    fs.mkdirSync(destDir, { recursive: true });
    console.log(`Created directory: ${destDir}`);
  }

  // 复制文件
  try {
    fs.copyFileSync(sourceFile, destFile);
    console.log(`✅ Copied ${executableName} to bin/ directory`);

    // 获取文件信息
    const stats = fs.statSync(sourceFile);
    const size = (stats.size / 1024).toFixed(2);
    console.log(`   File size: ${size} KB`);

    // 对于非Windows平台，设置可执行权限
    if (platform !== 'win32') {
      try {
        fs.chmodSync(destFile, 0o755); // rwxr-xr-x
        console.log(`✅ Set executable permissions (755) for ${executableName}`);
      } catch (error) {
        console.warn(`⚠️ Could not set executable permissions: ${error.message}`);
      }
    }

    // 创建平台无关的符号链接（仅限 Unix-like 系统）
    if (platform !== 'win32') {
      try {
        const symlinkPath = path.join(destDir, 'winfox');

        // 如果符号链接已存在，先删除
        if (fs.existsSync(symlinkPath)) {
          fs.unlinkSync(symlinkPath);
        }

        // 创建符号链接
        fs.symlinkSync(executableName, symlinkPath);
        console.log(`✅ Created symbolic link: winfox -> ${executableName}`);
      } catch (error) {
        console.warn(`⚠️ Could not create symbolic link: ${error.message}`);
      }
    }

    console.log('\n🎉 Build completed successfully!');
    console.log(`📁 The CLI executable is available at: ${destFile}`);

    // 使用说明
    console.log('\n📋 Usage instructions:');

    if (platform === 'win32') {
      console.log('1. To use globally, add this directory to your PATH:');
      console.log(`   ${destDir}`);
      console.log('\n2. Or run directly:');
      console.log(`   ${destFile} --help`);
      console.log('\n3. Test the CLI:');
      console.log(`   ${destFile} --version`);
    } else {
      console.log('1. To use globally, add to PATH:');
      console.log(`   export PATH="${destDir}:$PATH"`);
      console.log('\n2. Or create a system-wide symlink:');
      console.log(`   sudo ln -s ${destFile} /usr/local/bin/winfox`);
      console.log('\n3. Test the CLI:');
      console.log(`   ${destFile} --help`);
      console.log(`   winfox --version  # if symlink created`);
    }

    console.log('\n🔧 For cross-compilation to other platforms:');
    console.log('   cargo build --release --target=<target-triple>');
    console.log('   Example targets:');
    console.log('   - x86_64-pc-windows-gnu (Windows)');
    console.log('   - x86_64-apple-darwin (macOS Intel)');
    console.log('   - aarch64-apple-darwin (macOS Apple Silicon)');
    console.log('   - x86_64-unknown-linux-gnu (Linux)');

  } catch (error) {
    console.error(`❌ Error copying file: ${error.message}`);
    console.error('Stack trace:', error.stack);
    process.exit(1);
  }
}

// 执行主函数
main();
