<script setup lang="ts">
import { ref } from 'vue'

interface Feature {
  id: number
  title: string
  description: string
  icon: string
}

const features = ref<Feature[]>([
  { id: 1, title: '变量', description: '使用 $variable 定义可重用值', icon: '🎨' },
  { id: 2, title: '嵌套', description: '支持CSS选择器嵌套语法', icon: '📦' },
  { id: 3, title: '混合器', description: '使用 @mixin 创建可重用样式块', icon: '🔄' },
  { id: 4, title: '函数', description: '内置和自定义函数处理颜色和数值', icon: '⚙️' },
  { id: 5, title: '继承', description: '使用 @extend 共享样式规则', icon: '📝' },
  { id: 6, title: '导入', description: '模块化组织样式文件', icon: '📁' },
])

const activeFeature = ref<Feature>(features.value[0] as Feature)

const setActiveFeature = (feature: Feature) => {
  activeFeature.value = feature
}
</script>

<template>
  <div class="scss-demo">
    <div class="demo-header">
      <h2 class="demo-title">SCSS 功能展示</h2>
      <p class="demo-description">探索SCSS的强大功能</p>
    </div>

    <div class="demo-content">
      <div class="features-sidebar">
        <h3 class="sidebar-title">功能列表</h3>
        <ul class="feature-list">
          <li
            v-for="feature in features"
            :key="feature.id"
            :class="{ active: activeFeature.id === feature.id }"
            @click="setActiveFeature(feature)"
            class="feature-item"
          >
            <span class="feature-icon">{{ feature.icon }}</span>
            <span class="feature-title">{{ feature.title }}</span>
          </li>
        </ul>
      </div>

      <div class="feature-detail">
        <div class="detail-header">
          <h3 class="detail-title">
            <span class="detail-icon">{{ activeFeature.icon }}</span>
            {{ activeFeature.title }}
          </h3>
          <p class="detail-description">{{ activeFeature.description }}</p>
        </div>

        <div class="code-example">
          <h4 class="code-title">示例代码</h4>
          <pre class="code-block"><code>{{ getExampleCode(activeFeature.id) }}</code></pre>
        </div>

        <div class="visual-demo">
          <h4 class="visual-title">视觉效果</h4>
          <div :class="`demo-visual demo-feature-${activeFeature.id}`">
            <div class="visual-content">
              <p>SCSS {{ activeFeature.title }} 演示</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="demo-footer">
      <p class="footer-text">SCSS让CSS变得更强大、更易维护！</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
// 导入变量和混合器
@import '@/assets/styles/variables';
@import '@/assets/styles/mixins';

.scss-demo {
  @include card-base;
  margin: $spacing-xl 0;
  overflow: hidden;
}

.demo-header {
  background: linear-gradient(135deg, $primary-color, lighten($primary-color, 20%));
  color: white;
  padding: $spacing-xl $spacing-lg;
  text-align: center;

  .demo-title {
    font-size: 2.2rem;
    margin-bottom: $spacing-sm;
    color: white;
  }

  .demo-description {
    font-size: $font-size-large;
    opacity: 0.9;
  }
}

.demo-content {
  display: grid;
  grid-template-columns: 1fr;
  gap: $spacing-lg;
  padding: $spacing-lg;

  @include respond-to(md) {
    grid-template-columns: 300px 1fr;
  }
}

.features-sidebar {
  background-color: #f8f9fa;
  border-radius: $border-radius;
  padding: $spacing-lg;
  border: 1px solid $border-color;

  .sidebar-title {
    color: $primary-color;
    margin-bottom: $spacing-lg;
    padding-bottom: $spacing-sm;
    border-bottom: 2px solid $border-color;
  }
}

.feature-list {
  list-style: none;

  .feature-item {
    @include flex-center;
    gap: $spacing-sm;
    padding: $spacing-md;
    margin-bottom: $spacing-sm;
    border-radius: $border-radius;
    cursor: pointer;
    transition: all 0.3s ease;
    border: 1px solid transparent;

    &:hover {
      background-color: rgba($primary-color, 0.1);
      border-color: rgba($primary-color, 0.2);
      transform: translateX(5px);
    }

    &.active {
      background-color: rgba($primary-color, 0.15);
      border-color: $primary-color;
      font-weight: 600;
      color: $primary-color;
    }

    .feature-icon {
      font-size: 1.5rem;
    }

    .feature-title {
      font-size: $font-size-base;
    }
  }
}

.feature-detail {
  .detail-header {
    margin-bottom: $spacing-xl;

    .detail-title {
      @include flex-center;
      gap: $spacing-sm;
      color: $secondary-color;
      margin-bottom: $spacing-sm;

      .detail-icon {
        font-size: 1.8rem;
      }
    }

    .detail-description {
      color: #666;
      font-size: $font-size-large;
      line-height: 1.6;
    }
  }
}

.code-example {
  margin-bottom: $spacing-xl;

  .code-title {
    color: $info-color;
    margin-bottom: $spacing-md;
  }

  .code-block {
    background-color: #2d2d2d;
    color: #f8f8f2;
    padding: $spacing-lg;
    border-radius: $border-radius;
    overflow-x: auto;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: $font-size-small;
    line-height: 1.5;

    code {
      display: block;
    }
  }
}

.visual-demo {
  .visual-title {
    color: $warning-color;
    margin-bottom: $spacing-md;
  }
}

.demo-visual {
  @include flex-center;
  min-height: 200px;
  border-radius: $border-radius;
  padding: $spacing-xl;
  transition: all 0.5s ease;

  .visual-content {
    text-align: center;
    color: white;
    font-size: $font-size-large;
    font-weight: 600;
  }

  // 不同功能的视觉效果
  &.demo-feature-1 {
    background: linear-gradient(135deg, $primary-color, lighten($primary-color, 20%));
  }

  &.demo-feature-2 {
    background: linear-gradient(135deg, $secondary-color, lighten($secondary-color, 20%));
  }

  &.demo-feature-3 {
    background: linear-gradient(135deg, $danger-color, lighten($danger-color, 20%));
  }

  &.demo-feature-4 {
    background: linear-gradient(135deg, $warning-color, lighten($warning-color, 20%));
  }

  &.demo-feature-5 {
    background: linear-gradient(135deg, $info-color, lighten($info-color, 20%));
  }

  &.demo-feature-6 {
    background: linear-gradient(135deg, #1abc9c, lighten(#1abc9c, 20%));
  }
}

.demo-footer {
  background-color: #f8f9fa;
  padding: $spacing-lg;
  text-align: center;
  border-top: 1px solid $border-color;

  .footer-text {
    color: #666;
    font-size: $font-size-base;
    font-weight: 500;
  }
}
</style>

<script lang="ts">
export default {
  methods: {
    getExampleCode(featureId: number): string {
      const examples: Record<number, string> = {
        1: `// 变量定义
$primary-color: #3498db;
$font-size-base: 16px;
$spacing-md: 16px;

.button {
  background-color: $primary-color;
  font-size: $font-size-base;
  padding: $spacing-md;
}`,
        2: `// 嵌套语法
.card {
  padding: 16px;
  border-radius: 4px;

  .title {
    font-size: 24px;
    color: #333;

    &:hover {
      color: #3498db;
    }
  }

  .content {
    margin-top: 16px;
  }
}`,
        3: `// 混合器
@mixin flex-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

@mixin button-style($bg-color) {
  padding: 8px 16px;
  background-color: $bg-color;
  color: white;
  border: none;
  border-radius: 4px;

  &:hover {
    background-color: darken($bg-color, 10%);
  }
}

.container {
  @include flex-center;
}

.btn-primary {
  @include button-style(#3498db);
}`,
        4: `// 函数
$primary-color: #3498db;

// 内置函数
.dark-bg {
  background-color: darken($primary-color, 20%);
}

.light-bg {
  background-color: lighten($primary-color, 20%);
}

// 自定义函数
@function calculate-rem($px) {
  @return ($px / 16) * 1rem;
}

.text-large {
  font-size: calculate-rem(24px);
}`,
        5: `// 继承
%message-shared {
  border: 1px solid #ccc;
  padding: 10px;
  color: #333;
}

.success-message {
  @extend %message-shared;
  border-color: green;
  background-color: #d4edda;
}

.error-message {
  @extend %message-shared;
  border-color: red;
  background-color: #f8d7da;
}`,
        6: `// 导入
// _variables.scss
$primary-color: #3498db;
$secondary-color: #2ecc71;

// _mixins.scss
@mixin flex-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

// main.scss
@import 'variables';
@import 'mixins';

.container {
  @include flex-center;
  color: $primary-color;
}`,
      }

      return examples[featureId] || '// 示例代码'
    },
  },
}
</script>
