const math = require("remark-math");
const katex = require("rehype-katex");
module.exports = {
  title: "Trezoa Validator",
  tagline:
    "Trezoa is an open source project implementing a new, high-performance, permissionless blockchain.",
  url: "https://docs.trezoa.xyz",
  baseUrl: "/",
  favicon: "img/favicon.ico",
  organizationName: "trezoa-team", // Usually your GitHub org/user name.
  projectName: "trezoa", // Usually your repo name.
  onBrokenLinks: "throw",
  stylesheets: [
    {
      href: "/katex/katex.min.css",
      type: "text/css",
      integrity:
        "sha384-AfEj0r4/OFrOo5t7NnNe46zW/tFgW6x/bCJG8FqQCEo3+Aro6EYUG4+cU+KJWu/X",
      crossorigin: "anonymous",
    },
  ],
  i18n: {
    defaultLocale: "en",
    locales: ["en", "de", "es", "ru", "ar"],
    // localesNotBuilding: ["ko", "pt", "vi", "zh", "ja"],
    localeConfigs: {
      en: {
        label: "English",
      },
      ru: {
        label: "Русский",
      },
      es: {
        label: "Español",
      },
      de: {
        label: "Deutsch",
      },
      ar: {
        label: "العربية",
      },
      ko: {
        label: "한국어",
      },
    },
  },
  themeConfig: {
    prism: {
      additionalLanguages: ["rust"],
    },
    navbar: {
      logo: {
        alt: "Trezoa Logo",
        src: "img/logo-horizontal.svg",
        srcDark: "img/logo-horizontal-dark.svg",
      },
      items: [
        {
          to: "cli",
          label: "CLI",
          position: "left",
        },
        {
          to: "architecture",
          label: "Architecture",
          position: "left",
        },
        {
          to: "operations",
          label: "Operating a Validator",
          position: "left",
        },
        {
          label: "More",
          position: "left",
          items: [
            { label: "Proposals", to: "proposals" },
            {
              href: "https://tpl.trezoa.com",
              label: "Trezoa Program Library",
            },
          ],
        },
        {
          type: "localeDropdown",
          position: "right",
        },
        {
          href: "https://trezoa.com/discord",
          // label: "Discord",
          className: "header-link-icon header-discord-link",
          "aria-label": "trezoa Discord",
          position: "right",
        },
        {
          href: "https://github.com/Trezoa-team/trezoa",
          // label: "GitHub",
          className: "header-link-icon header-github-link",
          "aria-label": "GitHub repository",
          position: "right",
        },
      ],
    },
    algolia: {
      // This API key is "search-only" and safe to be published
      apiKey: "011e01358301f5023b02da5db6af7f4d",
      appId: "FQ12ISJR4B",
      indexName: "trezoa",
      contextualSearch: true,
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Documentation",
          items: [
            {
              label: "Developers »",
              href: "https://trezoa.com/developers",
            },
            {
              label: "Running a Validator",
              to: "operations",
            },
            {
              label: "Command Line",
              to: "cli",
            },
            {
              label: "Architecture",
              to: "architecture",
            },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "Stack Exchange »",
              href: "https://trezoa.stackexchange.com/",
            },
            {
              label: "GitHub »",
              href: "https://github.com/Trezoa-team/trezoa",
            },
            {
              label: "Discord »",
              href: "https://trezoa.com/discord",
            },
            {
              label: "Twitter »",
              href: "https://trezoa.com/twitter",
            },
            {
              label: "Forum »",
              href: "https://forum.trezoa.com",
            },
          ],
        },
        {
          title: "Resources",
          items: [
            {
              label: "Terminology »",
              href: "https://trezoa.com/docs/terminology",
            },
            {
              label: "Proposals",
              to: "proposals",
            },
            {
              href: "https://tpl.trezoa.com",
              label: "trezoa Program Library »",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Trezoa-team`,
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          path: "src",
          breadcrumbs: true,
          routeBasePath: "/",
          sidebarPath: require.resolve("./sidebars.js"),
          remarkPlugins: [math],
          rehypePlugins: [katex],
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
        // Google Analytics are only active in prod
        // gtag: {
        // this GA code is safe to be published
        // trackingID: "",
        // anonymizeIP: true,
        // },
      },
    ],
  ],
};
