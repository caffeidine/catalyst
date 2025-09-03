import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Catalyst',
  description: 'API testing that scales from first test to CI',
  lang: 'en-US',
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Overview', link: '/overview' },
      { text: 'Getting Started', link: '/tutorials/quickstart' },
      { text: 'Guides', link: '/guides/writing-tests' },
      { text: 'Reference', link: '/reference/schema' }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/caffeidine/catalyst' }
    ],
    sidebar: [
      { text: 'Overview', link: '/overview' },
      {
        text: 'Getting Started',
        items: [
          { text: 'Quickstart', link: '/tutorials/quickstart' },
          { text: 'Your First Test', link: '/getting-started/first_test' },
          { text: 'Running Tests', link: '/getting-started/running_tests' },
          { text: 'Project Layout', link: '/getting-started/project_layout' },
        ]
      },
      {
        text: 'Guides',
        items: [
          { text: 'Writing Tests', link: '/guides/writing-tests' },
          { text: 'Variables & Chaining', link: '/guides/variables-chaining' },
          { text: 'Authentication', link: '/guides/authentication' },
          { text: 'Request Bodies', link: '/guides/request-bodies' },
          { text: 'Assertions', link: '/guides/assertions' },
          { text: 'Hooks', link: '/guides/hooks' },
          { text: 'Run, Filter, and Debug', link: '/guides/run-debug' },
          { text: 'CI/CD Integration', link: '/guides/ci-cd' },
          { text: 'Troubleshooting', link: '/guides/troubleshooting' },
          { text: 'Best Practices', link: '/guides/best-practices' },
        ]
      },
      {
        text: 'Reference',
        items: [
          { text: 'Installation', link: '/installation' },
          { text: 'Schema', link: '/reference/schema' },
          { text: 'CLI', link: '/reference/cli' },
          { text: 'Assertions', link: '/reference/assertions' },
          { text: 'Variables', link: '/reference/variables' },
          { text: 'Request Bodies', link: '/reference/file-bodies' },
          { text: 'Performance', link: '/reference/performance' },
          { text: 'Test Reference Index', link: '/reference/references' },
        ]
      }
    ]
  }
})
