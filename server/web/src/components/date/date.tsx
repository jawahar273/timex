"use client";
import {
  useForm,
  SubmitHandler,
  UseFormRegister,
  FieldPath,
} from "react-hook-form";

import { EndOption, RepeatEvery, SetEventType, TimexEvent } from "@timex/types";
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
import { CalendarIcon } from "@radix-ui/react-icons";
import { cn } from "@timex/utils";
import { format } from "date-fns";
import { Calendar } from "../ui/calendar";
import { Popover, PopoverTrigger, PopoverContent } from "../ui/popover";
import axios from "axios";
import dayjs from "dayjs";

type DateProps = {
  setEvent: (details: SetEventType) => void;
};

function demoFill(data: TInputs) {
  switch (data.type) {
    default:
    case Types.EVERY_DAY: {
      data = {
        ...data,
        repeatEveryNumber: 1,
        repeatEvery: "day" as RepeatEvery,
        endOption: "never" as EndOption,
      };
      return data;
    }
    case Types.EVERY_WEEK: {
      data = {
        ...data,
        repeatEveryNumber: 1,
        repeatEvery: "week" as RepeatEvery,
        endOption: "never" as EndOption,
        weekDaysForRepeatEvery: [],
      };
      return data;
    }
    case Types.EVERY_MONTH: {
      return {
        ...data,
        repeatEveryNumber: 1,
        repeatEvery: "month" as RepeatEvery,
        endOption: "never",
        monthOptions: "onDay",
        onDayValueForMonth: 1,
      };
    }
    case Types.EVERY_MONTH_2: {
      return {
        ...data,
        repeatEveryNumber: 2,
        repeatEvery: "month" as RepeatEvery,
        endOption: "never",
        monthOptions: "onDay",
        onDayValueForMonth: 1,
      };
    }
  }
}

export function DateJsx(props: DateProps) {
  const form = useForm<TInputs>({
    defaultValues: {
      repeatEveryNumber: 1,
      repeatEveryType: RepeatEvery.Day,
    },
  });

  const onSubmit: SubmitHandler<TInputs> = async (data) => {
    // data.scheduledStartDateTime = typeof data.scheduledStartDateTime === 'date' ? data.scheduledStartDateTime.toISOString() : data.scheduledStartDateTime;
    const temp = demoFill(data);
    temp.scheduledStartDateTime = dayjs(temp.scheduledStartDateTime)
      .second(59)
      .minute(59)
      .hour(11)
      .toDate();

    const res = await axios.post<TimexEvent>(
      "http://localhost:8300/api/v1/schedule/",
      {
        details: demoFill(data),
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
    props.setEvent({ events: res.data });
    console.log(res);
  };

  // console.log("====", repeatEvery)

  return (
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
                    selected={field.value}
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
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
