"use client";

import { DateJsx, DateCalendar } from "@timex/components";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@timex/components/ui/resizable";
import dayjs from "dayjs";
import { useEffect, useState } from "react";
import { Event } from "react-big-calendar";
import { SetEventType } from "@timex/types";
import { Card, CardContent } from "@timex/components/ui/card";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@timex/components/ui/tabs";

import { string } from "zod";
import axios from "axios";
import { Highlighter, HighlighterLang } from "../highlight";

import init, {
  showDetailInDisplay,
} from "../../../public/pkg";
import { useToast } from "@timex/components/ui/use-toast";
import { Alert, AlertDescription, AlertTitle } from "../ui/alert";

async function Temp() {
  await init();
}

export default function Home() {
  const [dateTime, setDateTime] = useState<Record<string, Event[]>>();
  const [orignal, setOrignal] = useState<string>("");
  const { toast } = useToast()
  
  useEffect(() => {
    Temp()
  }, [])
  
  const setEvent = ({ events }: SetEventType) => {
    setOrignal(JSON.stringify(events.scheduledDateTime || [], null, 3));
    console.log('===>', JSON.stringify(events.scheduledDateTime || [], null, 3))
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

    setDateTime(y);
  }

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
              <DateJsx onDropDownChange={(event) => {
                toast({
                  title: showDetailInDisplay(event) ,
                  description: 'Selected options',
                })
              }} setEvent={setEvent} />

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
                      {/* {
                        orignal
                      } */}
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
