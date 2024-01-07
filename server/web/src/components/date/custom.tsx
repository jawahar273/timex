"use client";
import {
  useForm,
  SubmitHandler,
  UseFormRegister,
  UseFormReturn,
  FieldPath,
} from "react-hook-form";

import { RepeatEvery, ScheduleDetails } from "@timex/types";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@radix-ui/react-select";
import { string } from "zod";
import { TInputs } from "./types";

export function customDate({
  form,
  onSubmit,
}: {
  form: UseFormReturn<TInputs>;
  onSubmit: SubmitHandler<TInputs>;
}) {
  const repeatEvery = form.watch("repeatEvery");

  return (
    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
      {/* <FormField
          control={form.control}
          name="scheduledStartDateTime"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Scheduled start date</FormLabel>
              <FormControl>
                <Input type="date" placeholder="Schedule date" {...field} />
              </FormControl>
              <FormDescription>
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        /> */}

      <div className="grid grid-cols-2 gap-2">
        <FormField
          control={form.control}
          name="repeatEveryNumber"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Repeat Every Time(s)</FormLabel>
              <FormControl>
                <Input
                  className="col-span-2"
                  type="number"
                  min={1}
                  {...field}
                />
              </FormControl>
              <FormDescription>
                {/* This is your public display name. */}
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="repeatEvery"
          render={({ field }) => {
            return (
              <select {...field}>
                <option value={RepeatEvery.Day}>Day</option>
                <option value={RepeatEvery.Week}>week</option>
                <option value={RepeatEvery.Month}>Month</option>
                <option value={RepeatEvery.Year}>year</option>
              </select>
            );
          }}
        />
      </div>

      {repeatEvery === RepeatEvery.Week && (
        <div className="grid grid-cols-1 gap-2">
          <WeekDay
            name="weekDayForMonth"
            register={form.register}
            multiple={true}
          />
        </div>
      )}

      {repeatEvery === RepeatEvery.Month && (
        <div className="grid grid-cols-1 gap-2">
          <WeekDay
            name="weekDayForMonth"
            register={form.register}
            multiple={true}
          />
        </div>
      )}

      {/* <FormField
            control={form.control}
            name="repeatEvery"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Every</FormLabel>
                <Select onValueChange={field.onChange} defaultValue={field.value}>
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="Select a type" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                  <SelectItem value={RepeatEvery.Day}>Day</SelectItem>
                  <SelectItem value={RepeatEvery.Week}>{RepeatEvery.Week}</SelectItem>
                  <SelectItem value={RepeatEvery.Month}>{RepeatEvery.Month}</SelectItem>
                  <SelectItem value={RepeatEvery.Year}>{RepeatEvery.Year}</SelectItem>
                  </SelectContent>
                </Select>
                <FormDescription>
                  You can manage email addresses in your{" "}
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          /> */}
      <FormField
        control={form.control}
        name="endDate"
        render={({ field }) => (
          <FormItem>
            <FormLabel>End date</FormLabel>
            <FormControl>
              <Input type="date" placeholder="end date" {...field} />
            </FormControl>
            <FormDescription>
              {/* This is your public display name. */}
            </FormDescription>
            <FormMessage />
          </FormItem>
        )}
      />
      <Button type="submit">Submit</Button>
    </form>
  );
}

function WeekDay({
  name,
  multiple,
  register,
}: {
  name: FieldPath<TInputs>;
  register: UseFormRegister<TInputs>;
  multiple: boolean;
}) {
  return (
    <select {...register(name, { required: true })} multiple={multiple}>
      <option value="monday">monday</option>
      <option value="tuesday">tuesday</option>
      <option value="wednesday">wednesday</option>
      <option value="thursday">thursday</option>
      <option value="friday">friday</option>
      <option value="saturday">saturday</option>
      <option value="sunday">sunday</option>
    </select>
  );
}

function ToDisplayOnWeek({ register }: { register: UseFormRegister<TInputs> }) {
  return (
    <div>
      <WeekDay name="weekDayForMonth" register={register} multiple={false} />
    </div>
  );
}

//    <Form {...form}>

//       <form onSubmit={form.handleSubmit(onSubmit)}>
// <FormField
//           control={form.control}

// >

// </FormField>
// <input type="datetime-local" placeholder="schedule start date"  {...register("scheduledStartDateTime")} />
// <input type="number" min={1} defaultValue={1} placeholder="repeat every time" {...register("repeatEveryNumber")}  />
//  <select {...register("repeatEvery", { required: true })}>
//         <option value={RepeatEvery.Day}>Day</option>
//         <option value={RepeatEvery.Week}>week</option>
//         <option value={RepeatEvery.Month}>Month</option>
//         <option value={RepeatEvery.Year}>year</option>
//       </select>

// {
//  repeatEvery === RepeatEvery.Week && (
//   <ToDisplayOnWeek register={register} />
//  )
// }

// {
//  repeatEvery === RepeatEvery.Month && (
//   <WeekDay name="weekDayForYear" register={register} />
//  )
// }

// <input type="date"  placeholder="schedule end date" {...register("endDate")} />
// <button type="submit" >
// Submit
// </button>

//       </form>
//     </Form>
