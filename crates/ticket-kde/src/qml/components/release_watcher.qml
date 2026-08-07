import QtQuick

Timer {
  id: watcher

  property var root
  property var backend
  property string announcedVersion: ""

  signal updateDetected(var release)
  property var status: ({
      organization: "",
      organization_url: "",
      app_url: "",
      repository: "",
      current_version: "",
      latest_version: "",
      update_available: false,
      checked_at: null
  })

  interval: 3 * 60 * 60 * 1000
  repeat: true
  running: true
  triggeredOnStart: true
  onTriggered: watcher.check()

  Component.onCompleted: watcher.refreshStatus()

  function check() {
    watcher.backend.checkReleaseAsync()
  }

  function refreshStatus() {
    const response = watcher.root.parseResponse(watcher.backend.releaseStatus())
    if (response.ok) watcher.applyStatus(response.data)
  }

  function applyStatus(next) {
    watcher.status = next
    if (!next.update_available) return
    const version = String(next.latest_version || "")
    if (version === "" || version === watcher.announcedVersion) return
    watcher.announcedVersion = version
    watcher.updateDetected(next)
  }

  function handleResponse(responseJson) {
    const response = watcher.root.parseResponse(responseJson)
    if (response.ok) watcher.applyStatus(response.data)
  }
}
