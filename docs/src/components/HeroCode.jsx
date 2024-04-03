import clsx from 'clsx'
import Highlight, { defaultProps } from 'prism-react-renderer'
import { Fragment } from 'react'
import { TrafficLightsIcon } from '@/components/icons/TrafficLightsIcon'

export function HeroCode({ tabs, code, language }) {
  return (
    <div className="relative">
      <div className="absolute -top-px left-20 right-11 h-px bg-gradient-to-r from-sky-300/0 via-sky-300/70 to-sky-300/0"></div>
      <div className="absolute inset-0 rounded-2xl bg-gradient-to-tr from-sky-300 via-sky-300/70 to-blue-300 opacity-10"></div>
      <div className="relative rounded-2xl bg-[#0A101F]/80 ring-1 ring-white/10 backdrop-blur">
        <div className="absolute -top-px left-20 right-11 h-px bg-gradient-to-r from-sky-300/0 via-accent-300 to-sky-300/0 opacity-75"></div>
        <div className="absolute -bottom-px left-11 right-20 h-px bg-gradient-to-r from-blue-400/0 via-accent-400 to-blue-400/0 opacity-75"></div>
        <div className="pl-4 pt-4">
          <TrafficLightsIcon className="h-2.5 w-auto stroke-slate-500/30" />
          {tabs && tabs.length > 0 && (
            <div className="mt-4 flex space-x-2 text-xs">
              {tabs.map((tab) => (
                <div
                  key={tab.name}
                  className={clsx(
                    'flex h-6 rounded-full',
                    tab.isActive
                      ? 'from-accent-400/30 to-accent-400/30 bg-gradient-to-r via-accent-400 p-px font-medium text-accent-300'
                      : 'text-slate-500'
                  )}
                >
                  <div
                    className={clsx(
                      'flex items-center rounded-full px-2.5',
                      tab.isActive && 'bg-slate-800'
                    )}
                  >
                    {tab.name}
                  </div>
                </div>
              ))}
            </div>
          )}
          <div className="mt-6 flex items-start px-1 text-sm">
            <div
              aria-hidden="true"
              className="select-none border-r border-slate-300/5 pr-4 font-mono text-slate-600"
            >
              {Array.from({
                length: code.split('\n').length,
              }).map((_, index) => (
                <Fragment key={index}>
                  {(index + 1).toString().padStart(2, '0')}
                  <br />
                </Fragment>
              ))}
            </div>
            <Highlight
              {...defaultProps}
              code={code}
              language={language}
              theme={undefined}
            >
              {({ className, style, tokens, getLineProps, getTokenProps }) => (
                <pre
                  className={clsx(className, 'flex overflow-x-auto pb-6')}
                  style={style}
                >
                  <code className="px-4">
                    {tokens.map((line, lineIndex) => (
                      <div key={lineIndex} {...getLineProps({ line })}>
                        {line.map((token, tokenIndex) => (
                          <span
                            key={tokenIndex}
                            {...getTokenProps({ token })}
                          />
                        ))}
                      </div>
                    ))}
                  </code>
                </pre>
              )}
            </Highlight>
          </div>
        </div>
      </div>
    </div>
  )
}
