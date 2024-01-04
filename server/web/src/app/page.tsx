'use client'
import Image from 'next/image'

import { Date, DateCalendar } from '@timex/components'

 
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@timex/components/ui/resizable'
import dayjs from 'dayjs'
import { useCallback, useState } from 'react'
import { Event } from 'react-big-calendar'
import { SetEventType } from '@timex/types'


export default function Home() {
  
  const [dateTime, setDateTime] = useState<Event[]>();
  
  const setEvent = useCallback(({
    events
  }: SetEventType) => {
    
    setDateTime(
      events.scheduledDateTime?.map(event => {
        return {
          title: dayjs(event).format('lll'),
          start: dayjs(event).toDate(),
          end:  dayjs(event).toDate(),
        }
      })
    );
  }, []);
  
  
  return (
<main>
<ResizablePanelGroup direction="horizontal">
  <ResizablePanel defaultSize={25}><Date setEvent={setEvent} />
</ResizablePanel>
  <ResizableHandle withHandle />
  <ResizablePanel>
    {
      dateTime?.length ? (
        <DateCalendar cEvents={dateTime}  />
      ) : (
        <h1 className='flex justify-center'>
          No Schedule date time
        </h1>
      )
    }
  </ResizablePanel>
</ResizablePanelGroup>
       </main>
  )
}
