"use client";
import {
  useForm,
  SubmitHandler,
  UseFormRegister,
  FieldPath,
} from "react-hook-form";

import { EndOption, RepeatEvery, ScheduleDetails, SetEventType, TimexEvent } from "@timex/types";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { TInputs, Types } from "./types";
import { Input } from "../ui/input";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@radix-ui/react-select";
import Link from "next/link";
import { Button } from "../ui/button";
import { CalendarIcon, ExclamationTriangleIcon } from "@radix-ui/react-icons";
import { cn } from "@timex/utils";
import { format } from "date-fns";
import { Calendar } from "../ui/calendar";
import { Popover, PopoverTrigger, PopoverContent } from "../ui/popover";
import axios from "axios";
import dayjs from "dayjs";
import { useState } from "react";
import _ from "lodash";
import { Alert, AlertTitle, AlertDescription } from "../ui/alert";
import { string } from "zod";

import { find_schedule_date_time as findScheduleDateTime } from "asm"

type DateProps = {
  setEvent: (details: SetEventType) => void;
  onDropDownChange: (d: ScheduleDetails) => void;
};

function demoFill(type: Types) {
  let data = {}
  switch (type) {
    default:
    case Types.EVERY_DAY: {
      data = {

        repeatEveryNumber: 1,
        repeatEvery: "day" as RepeatEvery,
        endOption: "never" as EndOption,
      };
      return data;
    }
    case Types.EVERY_WEEK: {
      data = {

        repeatEveryNumber: 1,
        repeatEvery: "week" as RepeatEvery,
        endOption: "never" as EndOption,
        weekDaysForRepeatEvery: [],
      };
      return data;
    }
    case Types.EVERY_MONTH: {
      return {

        repeatEveryNumber: 1,
        repeatEvery: "month" as RepeatEvery,
        endOption: "never",
        monthOptions: "onDay",
        onDayValueForMonth: 1,
      };
    }
    case Types.EVERY_MONTH_2: {
      return {

        repeatEveryNumber: 2,
        repeatEvery: "month" as RepeatEvery,
        endOption: "never",
        monthOptions: "onDay",
        onDayValueForMonth: 1,
      };
    }
    case Types.EVERY_MONTH_LAST_DAY : {
      return {


        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "onThe",
        "endDate":         dayjs()
        .add(12, "months")
        .format("YYYY-MM-DDT11:59:00.000Z"),
        "monthOptions": "onDay",
        "onDayValueForMonth": 31,
      }
    }
    
    case Types.FOR_EVERY_MONTH_1ST_DAY: {
      return {

        "repeatEveryNumber": 1,
        "repeatEvery": "month",
        "endOption": "never",
        "monthOptions": "onDay",
        "onDayValueForMonth": 1,
      }
    }
  }
}

export function DateJsx({setEvent, onDropDownChange}: DateProps) {
  
  const [loading, setLoading] = useState(false);
  const [errorObj, setHasError] = useState<Error>();
  
  const form = useForm<TInputs>({
    defaultValues: {
      repeatEveryNumber: 1,
      repeatEveryType: RepeatEvery.Day,
    },
  });
  
  const preProcess = (data: TInputs): TInputs => {
    const temp = {
      ...data,
      ...demoFill(data.type),
    };
    temp.scheduledStartDateTime = dayjs(temp.scheduledStartDateTime)
      .second(59)
      .minute(59)
      .hour(11)
      .toDate();
    return temp;
  }

  const onSubmit: SubmitHandler<TInputs> = async (data) => {
  try {
        const temp = preProcess(data)
      temp.scheduledStartDateTime = temp.scheduledStartDateTime.toString()
    const params =     {
      details: temp,
      previousScheduleDate: dayjs(
        dayjs(temp.scheduledStartDateTime)
          .subtract(1, "day")
          .format("YYYY-MM-DDT11:59:00.000Z")
      ).toISOString(),
      startDate: dayjs()
        .startOf("month")
        .add(1, "day")
        .format("YYYY-MM-DDT00:00:00.000Z"),
      endDate: dayjs()
        .add(14, "months")
        .endOf("month")
        .format("YYYY-MM-DDT00:00:00.000Z"),
    }
      const res = findScheduleDateTime(
        params.details,
        params.previousScheduleDate,
        params.startDate,
        params.endDate
      )
      console.log(res)
  setEvent({ events: { scheduledDateTime: res } as TimexEvent});
      
    }
  catch (error) {
     console.error("unexpected error from webassemby", error) 
    }
    

  // setEvent({ events: res.data });

    
  }
  
  const onSubmitWithApi: SubmitHandler<TInputs> = async (data) => {
    // data.scheduledStartDateTime = typeof data.scheduledStartDateTime === 'date' ? data.scheduledStartDateTime.toISOString() : data.scheduledStartDateTime;
    setLoading(true);
    const temp = preProcess(data)

const API_DOMAIN = 'https://timex.up.railway.app/';
try {
  const res = await axios.post<TimexEvent>(
    new URL ( "/api/v1/schedule/", API_DOMAIN).href,
    {
      details: temp,
      previousScheduleDate: dayjs(
        dayjs(temp.scheduledStartDateTime)
          .subtract(1, "day")
          .format("YYYY-MM-DDT11:59:00.000Z")
      ).toISOString(),
      startDate: dayjs()
        .startOf("month")
        .add(1, "day")
        .format("YYYY-MM-DDT00:00:00.000Z"),
      endDate: dayjs()
        .add(14, "months")
        .endOf("month")
        .format("YYYY-MM-DDT00:00:00.000Z"),
    },
    {
      headers: {
        "Content-Type": "application/json",
      },
    }
  );
  setEvent({ events: res.data });
  console.log(res);
} catch (error) {
  setHasError(error as any)
} finally {
  setLoading(false)
}

  };

  // console.log("====", repeatEvery)

  return (
    <div>
  {
    errorObj && 
    <Alert variant="destructive">
      <ExclamationTriangleIcon className="h-4 w-4" />
      <AlertTitle>Error</AlertTitle>
      <AlertDescription>
        {String(errorObj)}
      </AlertDescription>
    </Alert>
  }      
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8 pt-3">
        <FormField
          control={form.control}
          name="scheduledStartDateTime"
          rules={{
            required: true,
          }}
          render={({ field }) => (
            <FormItem className="flex flex-col">
              <FormLabel>Scheduled date</FormLabel>
              <Popover>
                <PopoverTrigger asChild>
                  <FormControl>
                    <Button
                      variant={"outline"}
                      className={cn(
                        "w-[240px] pl-3 text-left font-normal",
                        !field.value && "text-muted-foreground"
                      )}
                    >
                      {field.value ? (
                        format(field.value, "PPP")
                      ) : (
                        <span>Pick a date</span>
                      )}
                      <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                    </Button>
                  </FormControl>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0" align="start">
                  <Calendar
                    mode="single"
                    selected={field.value as Date}
                    onSelect={field.onChange}
                    defaultMonth={dayjs().toDate()}
                    // disabled={(date) => date > new Date() || date < new Date("1900-01-01")}
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
              <FormDescription></FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="type"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Demo options: </FormLabel>
              <FormControl>
                <select
                  {...field}
                  defaultValue={Types.EVERY_DAY}
                  multiple={false}
                >

                  <option value={Types.EVERY_DAY}>{Types.EVERY_DAY}</option>
                  <option value={Types.EVERY_WEEK}>{Types.EVERY_WEEK}</option>
                  <option value={Types.EVERY_MONTH}>{Types.EVERY_MONTH}</option>
                  <option value={Types.EVERY_MONTH_2}>
                    {Types.EVERY_MONTH_2}
                  </option>
                  <option value={Types.FOR_EVERY_MONTH_1ST_DAY}>
                    {Types.FOR_EVERY_MONTH_1ST_DAY}
                  </option>
                  <option value={Types.EVERY_MONTH_LAST_DAY}>
                    {Types.EVERY_MONTH_LAST_DAY}
                  </option>
                </select>
              </FormControl>
              <FormDescription></FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        {/* <FormField
          control={form.control}
          name="endDate"
          render={({ field }) => (
            <FormItem>
              <FormLabel>End date</FormLabel>
              <FormControl>
                <Input type="date" placeholder="end date" {...field} />
              </FormControl>
              <FormDescription>
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        /> */}
        <span className="space-x-1">
        <Button type="button"
        variant="outline"
        onClick={async () => {
          try {
            const status = await form.trigger()
            const temp = preProcess(form.getValues());
            if (status) {  
              temp.scheduledStartDateTime = dayjs(temp.scheduledStartDateTime).toISOString()            
              onDropDownChange(
                temp,
              )
            }
          } catch (error) {
            console.error("validation failed", error)
          }

        }}
        >Check</Button>
        <Button type="submit" disabled={loading}>Submit</Button>
        </span>
      </form>
    </Form>
    </div>
  );
}
