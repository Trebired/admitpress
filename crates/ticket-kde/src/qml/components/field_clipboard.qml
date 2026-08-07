pragma Singleton

import QtQml

QtObject {
  id: fieldClipboard

  property var fields: []
  property string sourceTicketId: ""
  property string sourceTemplateKind: ""
  readonly property bool hasFields: fields.length > 0

  function store(fieldList, ticketId, templateKind) {
    fields = fieldList || []
    sourceTicketId = String(ticketId || "")
    sourceTemplateKind = String(templateKind || "")
  }

  function clear() {
    fields = []
    sourceTicketId = ""
    sourceTemplateKind = ""
  }
}
