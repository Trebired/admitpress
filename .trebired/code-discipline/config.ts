import { defineCodeDisciplineConfig } from "@trebired/code-discipline";

export default defineCodeDisciplineConfig({
  ignore: {
    entries: [],
    use_gitignore: true,
  },
  rules: {
    formatting: {},
    bannedFiles: {
      patterns: [
        { glob: "**/*.spec.ts" },
        { glob: "**/*.spec.tsx" },
        { glob: "**/*fixture*" },
        { glob: "**/*fixture*/**" },
        { glob: "**/*ticketforge*" },
      ],
    },
    bannedPatterns: {
      patterns: [
        {
          value: "AdmitPress",
          allowedFiles: [
            "Cargo.toml",
            "crates/ticket-kde/Cargo.toml",
            "package.json",
            "packaging/fedora/app.ticket.native.desktop",
            "packaging/fedora/app.ticket.native.metainfo.xml",
          ],
        },
        {
          value: "admitpress",
          allowedFiles: [
            "Cargo.toml",
            "crates/ticket-kde/Cargo.toml",
            "package.json",
            "packaging/fedora/app.ticket.native.desktop",
            "packaging/fedora/app.ticket.native.metainfo.xml",
          ],
        },
        { value: "Trebired", allowedFiles: ["package.json"] },
        { value: "trebired", allowedFiles: ["package.json"] },
      ],
    },
    removeComments: {},
    minDeclarationName: {},
    maxCharactersPerLine: {},
    structuralBlankLines: {},
    minFileLines: {},
    maxFileLines: { max: 350 },
    maxFunctionLines: { max: 50 },
    redundantPathSegments: {
      separators: ["_", "-"],
    },
    dry: {},
  },
});
