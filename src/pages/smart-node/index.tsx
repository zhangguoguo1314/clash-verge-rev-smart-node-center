import {
  DashboardRounded,
  GroupRounded,
  HistoryRounded,
  HubRounded,
  SettingsRounded,
  SpeedRounded,
} from '@mui/icons-material'
import { Tab, Tabs } from '@mui/material'
import { Suspense, lazy, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { BasePage } from '@/components/base'
import { useSNC } from '@/providers/smart-node'

const LazyDashboard = lazy(() =>
  import('./dashboard').then((m) => ({
    default: m.SmartNodeDashboard,
  })),
)
const LazyPool = lazy(() =>
  import('./pool').then((m) => ({
    default: m.SmartNodePool,
  })),
)
const LazyGroups = lazy(() =>
  import('./groups').then((m) => ({
    default: m.SmartNodeGroups,
  })),
)
const LazySpeedTest = lazy(() =>
  import('./speed-test').then((m) => ({
    default: m.SmartNodeSpeedTest,
  })),
)
const LazyHistory = lazy(() =>
  import('./history').then((m) => ({
    default: m.SmartNodeHistory,
  })),
)
const LazySettings = lazy(() =>
  import('./settings').then((m) => ({
    default: m.SmartNodeSettings,
  })),
)

const TAB_KEYS = [
  'dashboard',
  'pool',
  'groups',
  'speed-test',
  'history',
  'settings',
] as const

type TabKey = (typeof TAB_KEYS)[number]

const SmartNodePage = () => {
  const { t } = useTranslation('smartNode')
  const { enabled } = useSNC()
  const [tab, setTab] = useState<TabKey>('dashboard')

  const tabs = useMemo(
    () => [
      {
        key: 'dashboard' as TabKey,
        label: t('tabs.dashboard'),
        icon: <DashboardRounded />,
      },
      {
        key: 'pool' as TabKey,
        label: t('tabs.pool'),
        icon: <HubRounded />,
      },
      {
        key: 'groups' as TabKey,
        label: t('tabs.groups'),
        icon: <GroupRounded />,
      },
      {
        key: 'speed-test' as TabKey,
        label: t('tabs.speedTest'),
        icon: <SpeedRounded />,
      },
      {
        key: 'history' as TabKey,
        label: t('tabs.history'),
        icon: <HistoryRounded />,
      },
      {
        key: 'settings' as TabKey,
        label: t('tabs.settings'),
        icon: <SettingsRounded />,
      },
    ],
    [t],
  )

  if (!enabled) return null

  const renderContent = () => {
    switch (tab) {
      case 'dashboard':
        return <LazyDashboard />
      case 'pool':
        return <LazyPool />
      case 'groups':
        return <LazyGroups />
      case 'speed-test':
        return <LazySpeedTest />
      case 'history':
        return <LazyHistory />
      case 'settings':
        return <LazySettings />
    }
  }

  return (
    <BasePage title={t('title')}>
      <Tabs
        value={tab}
        onChange={(_, v) => setTab(v as TabKey)}
        variant="scrollable"
        scrollButtons="auto"
        sx={{ mb: 2 }}
      >
        {tabs.map((item) => (
          <Tab
            key={item.key}
            value={item.key}
            label={item.label}
            icon={item.icon}
            iconPosition="start"
          />
        ))}
      </Tabs>
      <Suspense fallback={null}>{renderContent()}</Suspense>
    </BasePage>
  )
}

export default SmartNodePage
