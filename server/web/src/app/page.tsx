"use client";
import Image from "next/image";

import { DateJsx, DateCalendar } from "@timex/components";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@timex/components/ui/resizable";
import dayjs from "dayjs";
import { useCallback, useState } from "react";
import { Event } from "react-big-calendar";
import { SetEventType } from "@timex/types";
import { Highlighter, HighlighterLang } from "@timex/components/highlight";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "@radix-ui/react-accordion";
import { Card, CardContent } from "@timex/components/ui/card";
import { BlendingModeIcon as BadgeIcon } from "@radix-ui/react-icons";
import {
  Alert,
  AlertTitle,
  AlertDescription,
} from "@timex/components/ui/alert";

export default function Home() {
  const [dateTime, setDateTime] = useState<Event[]>();
  const [orignal, setOrignal] = useState<string>("");

  const setEvent = useCallback(({ events }: SetEventType) => {
    setOrignal(JSON.stringify(events.scheduledDateTime, null, 3));
    setDateTime(
      events.scheduledDateTime?.map((event) => {
        return {
          title: dayjs(event).format("lll"),
          start: dayjs(event).toDate(),
          end: dayjs(event).toDate(),
        };
      })
    );
  }, []);

  return (
    <main className="m-2">
      <h1 className="text-gray-900 text-4xl">Demo</h1>
      <h4 className="text-1xl">
        This is demo site which may or may not work work properly
      </h4>
      <Card>
        <CardContent>
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel defaultSize={25}>
              <DateJsx setEvent={setEvent} />

              <Accordion type="single" collapsible className="w-full p-2">
                <AccordionItem value="item-1">
                  <AccordionTrigger>
                    <div className="flex flex-col">
                      <BadgeIcon className="w-4 h-4" />
                      Payload
                    </div>
                  </AccordionTrigger>
                  <AccordionContent>
                    {orignal && (
                      <Highlighter
                        content={orignal}
                        language={HighlighterLang.JSON}
                      />
                    )}
                  </AccordionContent>
                </AccordionItem>
              </Accordion>
            </ResizablePanel>
            <ResizableHandle withHandle />
            <ResizablePanel>
              {dateTime?.length ? (
                <DateCalendar cEvents={dateTime} />
              ) : (
                <h1 className="flex justify-center">No Schedule date time</h1>
              )}
            </ResizablePanel>
          </ResizablePanelGroup>
        </CardContent>
      </Card>
    </main>
  );
}
