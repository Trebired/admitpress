#pragma once

#include <QtCore/QString>
#include <QtGui/QGuiApplication>
#include <QtGui/QIcon>

inline void applyWindowIcon(const QString &resourcePath)
{
    QGuiApplication::setWindowIcon(QIcon(resourcePath));
}
