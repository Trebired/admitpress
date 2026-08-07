import QtQuick

Timer {
  id: scheduler

  property var root
  property var backend
  property var status: ({ last_backup_at: "", count: 0, total_bytes: 0, directory: "" })
  property bool working: false
  property string lastError: ""

  interval: 3 * 60 * 60 * 1000
  repeat: true
  running: true
  triggeredOnStart: false
  onTriggered: scheduler.backupIfDue()

  function backupIfDue() {
    if (scheduler.working) return
    scheduler.working = true
    scheduler.backend.backupIfDueAsync()
  }

  function backupNow() {
    if (scheduler.working) return
    scheduler.working = true
    scheduler.backend.backupNowAsync()
  }

  function refreshStatus() {
    const response = scheduler.root.parseResponse(scheduler.backend.backupStatus())
    if (response.ok) scheduler.status = response.data
  }

  function handleResponse(operation, responseJson) {
    scheduler.working = false
    const response = scheduler.root.parseResponse(responseJson)
    if (!response.ok) {
      scheduler.lastError = response.error || LocaleText.tr("The backup failed.")
      if (operation === "backup_now") scheduler.root.showPassiveNotification(scheduler.lastError)
      return
    }
    scheduler.lastError = ""
    if (response.data.status) scheduler.status = response.data.status
    if (operation === "backup_now") {
      scheduler.root.showPassiveNotification(LocaleText.tr("Backup created"))
    }
  }
}
