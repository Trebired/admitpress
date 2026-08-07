pragma Singleton
import QtQuick

QtObject {
  id: appIcons

  property var backend: null
  readonly property bool preferGnome: backend ? backend.prefersGnomeIcons() : false

  readonly property var kdeNames: ({
      add: "list-add",
      alignHorizontal: "align-horizontal-center",
      alignVertical: "align-vertical-center",
      backup: "backup",
      calendar: "view-calendar",
      cancel: "dialog-cancel",
      clear: "edit-clear",
      copy: "edit-copy",
      "delete": "edit-delete",
      details: "documentinfo",
      edit: "document-edit",
      error: "dialog-error",
      "export": "document-export",
      folder: "folder",
      folderOpen: "folder-open",
      font: "preferences-desktop-font",
      groupItems: "view-list-details",
      home: "go-home",
      image: "insert-image",
      link: "link",
      "import": "document-import",
      money: "view-currency-list",
      moveHandle: "transform-move",
      next: "go-next",
      number: "format-list-ordered",
      ok: "dialog-ok",
      overflow: "overflow-menu",
      paste: "edit-paste",
      preview: "view-preview",
      previous: "go-previous",
      price: "view-currency-list",
      printPreview: "document-print-preview",
      printer: "printer",
      recorded: "view-financial-list",
      redo: "edit-redo",
      refresh: "view-refresh",
      remove: "value-decrease",
      reset: "edit-reset",
      save: "document-save",
      secure: "security-high",
      settings: "settings-configure",
      splitView: "view-split-left-right",
      staticText: "insert-text",
      today: "go-jump-today",
      undo: "edit-undo",
      wizard: "tools-wizard"
  })

  readonly property var gnomeNames: ({
      add: "list-add-symbolic",
      alignHorizontal: "format-justify-center-symbolic",
      alignVertical: "format-justify-fill-symbolic",
      backup: "drive-harddisk-symbolic",
      calendar: "x-office-calendar-symbolic",
      cancel: "process-stop-symbolic",
      clear: "edit-clear-symbolic",
      copy: "edit-copy-symbolic",
      "delete": "user-trash-symbolic",
      details: "dialog-information-symbolic",
      edit: "document-edit-symbolic",
      error: "dialog-error-symbolic",
      "export": "document-send-symbolic",
      folder: "folder-symbolic",
      folderOpen: "folder-open-symbolic",
      font: "font-x-generic-symbolic",
      groupItems: "view-list-symbolic",
      home: "go-home-symbolic",
      image: "insert-image-symbolic",
      link: "web-browser-symbolic",
      "import": "folder-download-symbolic",
      money: "accessories-calculator-symbolic",
      moveHandle: "list-drag-handle-symbolic",
      next: "go-next-symbolic",
      number: "view-list-ordered-symbolic",
      ok: "object-select-symbolic",
      overflow: "view-more-symbolic",
      paste: "edit-paste-symbolic",
      preview: "view-reveal-symbolic",
      previous: "go-previous-symbolic",
      price: "accessories-calculator-symbolic",
      printPreview: "document-print-symbolic",
      printer: "printer-symbolic",
      recorded: "view-list-bullet-symbolic",
      redo: "edit-redo-symbolic",
      refresh: "view-refresh-symbolic",
      remove: "list-remove-symbolic",
      reset: "view-refresh-symbolic",
      save: "document-save-symbolic",
      secure: "security-high-symbolic",
      settings: "preferences-system-symbolic",
      splitView: "view-dual-symbolic",
      staticText: "text-x-generic-symbolic",
      today: "go-jump-symbolic",
      undo: "edit-undo-symbolic",
      wizard: "applications-utilities-symbolic"
  })

  readonly property var current: preferGnome ? gnomeNames : kdeNames

  function pick(role) {
    return appIcons.current[role] || appIcons.kdeNames[role] || ""
  }
}
