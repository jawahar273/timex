"use client";
import Image from "next/image";
import dynamic from "next/dynamic";

import { DateJsx, DateCalendar } from "@timex/components";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@timex/components/ui/resizable";
import dayjs from "dayjs";
import { ReactNode, useCallback, useState } from "react";
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
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@timex/components/ui/tabs";
import { Button } from "@timex/components/ui/button";

import { string } from "zod";

// import timexW, {
//   showDetailInDisplay,
//   initSync,
//   SyncInitInput,
// } from "../../pkg/timex";
import { groupBy } from "lodash";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogTitle,
  DialogDescription,
} from "@radix-ui/react-dialog";
import { Label } from "@radix-ui/react-label";
import { DialogHeader, DialogFooter } from "@timex/components/ui/dialog";
import { Input } from "@timex/components/ui/input";

// async function Temp() {
//   const wasm = await timexW("../../pkg/timex");

//   const details = {
//     scheduledStartDateTime: "2023-12-10T09:08:44.939Z",
//     repeatEveryNumber: 1,
//     repeatEvery: "week",
//     endOption: "never",
//     weekDaysForRepeatEvery: [],
//   };

//   let temp = {};
//   Object.keys(details).forEach((k) => {
//     const newK = k.replace(/(\_\w)/g, (m) => m[1].toUpperCase());
//     temp[newK as keyof typeof string] = temp[k as keyof typeof string];
//     delete details[k as keyof typeof string];
//   });

//   console.log(wasm.showDetailInDisplay(temp as any));
// }

export default function Home() {
  const [dateTime, setDateTime] = useState<Record<string, Event[]>>();
  const [orignal, setOrignal] = useState<string>("");

  const setEvent = useCallback(({ events }: SetEventType) => {
    setOrignal(JSON.stringify(events.scheduledDateTime, null, 3));
    let y: Record<string, Event[]> = {};
    events.scheduledDateTime.forEach((r) => {
      let u = dayjs(r);
      let value = {
        title: dayjs(r).format("lll"),
        start: dayjs(r).toDate(),
        end: dayjs(r).toDate(),
      };
      const key = `${u.year()}-${u.month() + 1}`;
      if (!y[key as keyof typeof string]) {
        y[key as keyof typeof string] = [value];
      } else {
        y[key as keyof typeof string].push(value);
      }
    });
    console.log("$$$$$$$$$$$$$$$$", y);
    setDateTime(y);
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
            <ResizablePanel defaultSize={15}>
              <DateJsx setEvent={setEvent} />
              {/* 
              <div className="flex flex-col">
                <Dialog>
                  <DialogTrigger asChild>
                    <Button> View Raw Payload</Button>
                  </DialogTrigger>
                  <DialogContent className="sm:max-w-[425px]">
                    <DialogHeader>
                      <DialogTitle>Payload</DialogTitle>
                      <DialogDescription></DialogDescription>
                    </DialogHeader>
                    <div className="grid gap-4 py-4">
                      {orignal && (
                        <Highlighter
                          content={orignal}
                          language={HighlighterLang.JSON}
                        />
                      )}
                    </div>
                  </DialogContent>
                </Dialog>
              </div> */}
            </ResizablePanel>
            <ResizableHandle withHandle />
            <ResizablePanel className="p-2">
              {dateTime && Object.keys(dateTime).length ? (
                <>
                  <Tabs defaultValue="account" className="w-100">
                    <TabsList>
                      <TabsTrigger key={"calendar1"} value={"calender"}>
                        Calender
                      </TabsTrigger>
                      <TabsTrigger key={"payload"} value={"payload"}>
                        Payload
                      </TabsTrigger>
                    </TabsList>
                    <TabsContent value="payload">
                      {orignal && (
                        <Highlighter
                          content={orignal}
                          language={HighlighterLang.JSON}
                        />
                      )}
                    </TabsContent>
                    <TabsContent value="calender">
                      <div>
                        {/* calender */}
                        <Tabs orientation="vertical">
                          <TabsList className="grid w-full grid-cols-12">
                            {Object.keys(dateTime).map((key: string) => {
                              return (
                                <TabsTrigger key={key} value={key}>
                                  {key}
                                </TabsTrigger>
                              );
                            })}
                          </TabsList>
                          {Object.keys(dateTime).map((key: string) => {
                            const con = dateTime[key];
                            const d = dayjs(new Date(`${key}-01`));
                            return (
                              <TabsContent key={key} value={key}>
                                <DateCalendar
                                  defaultDate={d.toDate()}
                                  cEvents={con}
                                />
                              </TabsContent>
                            );
                          })}
                        </Tabs>
                      </div>
                    </TabsContent>
                  </Tabs>
                </>
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
