import QtQuick
import "../components"

QtObject {
  id: days

  required property var root
  required property var backend
  required property var page

  function dayBreakdownText(day) {
    if (!day || !day.subtickets || day.subtickets.length === 0) return LocaleText.tr("No sales")
    const parts = []
    for (let subticket of day.subtickets) {
      parts.push(subticket.quantity + "x " + root.formatMoney(subticket.amount_minor, subticket.currency))
    }
    return parts.join(" | ")
  }

  function daySummaries() {
    return page.summary && page.summary.day_summaries ? page.summary.day_summaries : []
  }

  function daySelected(localDate) {
    page.selectedDayRevision
    return !!page.selectedDayMap[localDate]
  }

  function setDaySelected(localDate, selected) {
    const next = Object.assign({}, page.selectedDayMap)
    if (selected) next[localDate] = true
    else delete next[localDate]
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function selectAllDays() {
    const next = {}
    for (let day of daySummaries()) next[day.local_date] = true
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function clearSelectedDays() {
    page.selectedDayMap = ({})
    page.selectedDayRevision += 1
  }

  function allDaysSelected() {
    page.selectedDayRevision
    const summaries = daySummaries()
    if (summaries.length === 0) return false
    for (let day of summaries) {
      if (!page.selectedDayMap[day.local_date]) return false
    }
    return true
  }

  function selectedDayCount() {
    page.selectedDayRevision
    let count = 0
    for (let day of daySummaries()) {
      if (page.selectedDayMap[day.local_date]) count += 1
    }
    return count
  }

  function selectedDayTicketCount() {
    page.selectedDayRevision
    let count = 0
    for (let day of daySummaries()) {
      if (page.selectedDayMap[day.local_date]) count += Number(day.ticket_count || 0)
    }
    return count
  }

  function selectedDayTotalMinor() {
    page.selectedDayRevision
    let total = 0
    for (let day of daySummaries()) {
      if (page.selectedDayMap[day.local_date]) total += Number(day.total_minor || 0)
    }
    return total
  }

  function selectedDayCurrency() {
    page.selectedDayRevision
    for (let day of daySummaries()) {
      if (page.selectedDayMap[day.local_date]) return day.currency || ""
    }
    return ""
  }

  function selectedDaySummaryText() {
    const count = selectedDayCount()
    if (count === 0) return LocaleText.tr("No days selected")
    const tickets = TextFormat.ticketCount(selectedDayTicketCount())
    const money = root.formatMoney(selectedDayTotalMinor(), selectedDayCurrency())
    return count + " days, " + tickets + ", " + money
  }

  function cleanupSelectedDays() {
    const next = {}
    for (let day of daySummaries()) {
      if (page.selectedDayMap[day.local_date]) next[day.local_date] = true
    }
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function switchToDate(localDateValue) {
    page.localDate = localDateValue
    page.load()
  }
}
