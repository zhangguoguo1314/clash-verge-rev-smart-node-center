import { Box } from '@mui/material'
import { useMemo } from 'react'

import { VirtualList } from '@/components/base'
import type { SmartNode } from '@/types/smart-node'

interface NodeListProps {
  nodes: SmartNode[]
  selectedUid: string | null
  onSelect: (uid: string | null) => void
}

const ITEM_HEIGHT = 52

export const NodeList = ({
  nodes,
  selectedUid,
  onSelect,
}: NodeListProps) => {
  const sortedNodes = useMemo(() => {
    return [...nodes].sort((a, b) => {
      if (a.status !== b.status) {
        const order: Record<string, number> = {
          healthy: 0,
          unknown: 1,
          unhealthy: 2,
        }
        return (order[a.status] ?? 1) - (order[b.status] ?? 1)
      }
      return b.score - a.score
    })
  }, [nodes])

  if (sortedNodes.length === 0) {
    return (
      <Box
        sx={{
          height: 200,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'text.secondary',
        }}
      >
        No nodes
      </Box>
    )
  }

  return (
    <Box sx={{ height: '50vh', overflow: 'hidden' }}>
      <VirtualList
        count={sortedNodes.length}
        estimateSize={ITEM_HEIGHT}
        getItemKey={(index) => sortedNodes[index].uid}
        renderItem={(index) => {
          const node = sortedNodes[index]
          const isSelected = node.uid === selectedUid
          return (
            <Box
              onClick={() =>
                onSelect(isSelected ? null : node.uid)
              }
              sx={{
                height: ITEM_HEIGHT,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                px: 2,
                cursor: 'pointer',
                bgcolor: isSelected
                  ? 'action.selected'
                  : 'transparent',
                borderBottom: 1,
                borderColor: 'divider',
                '&:hover': {
                  bgcolor: isSelected
                    ? 'action.selected'
                    : 'action.hover',
                },
              }}
            >
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, minWidth: 0, flex: 1 }}>
                <Box
                  sx={{
                    width: 8,
                    height: 8,
                    borderRadius: '50%',
                    flexShrink: 0,
                    bgcolor:
                      node.status === 'healthy'
                        ? 'success.main'
                        : node.status === 'unhealthy'
                          ? 'error.main'
                          : 'grey.400',
                  }}
                />
                <Box sx={{ minWidth: 0, flex: 1 }}>
                  <Box
                    sx={{
                      typography: 'body2',
                      overflow: 'hidden',
                      textOverflow: 'ellipsis',
                      whiteSpace: 'nowrap',
                    }}
                  >
                    {node.name}
                  </Box>
                  <Box
                    sx={{
                      typography: 'caption',
                      color: 'text.secondary',
                      overflow: 'hidden',
                      textOverflow: 'ellipsis',
                      whiteSpace: 'nowrap',
                    }}
                  >
                    {node.protocol}://{node.address}:{node.port}
                  </Box>
                </Box>
              </Box>
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexShrink: 0 }}>
                <Box sx={{ typography: 'caption', color: 'text.secondary' }}>
                  {node.latency_ms != null
                    ? `${node.latency_ms}ms`
                    : '-'}
                </Box>
                <Box sx={{ typography: 'body2', fontWeight: 'bold', minWidth: 36, textAlign: 'right' }}>
                  {node.score}
                </Box>
              </Box>
            </Box>
          )
        }}
      />
    </Box>
  )
}
