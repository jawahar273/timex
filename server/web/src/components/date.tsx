'use client'
import { useForm, SubmitHandler, UseFormRegister, UseFormReturn, FieldPath } from "react-hook-form"

import { RepeatEvery, ScheduleDetails } from '@timex/types'

type DateProps = {}

type Inputs = ScheduleDetails;

export function Date(props: DateProps) {
  const {
    register,
    handleSubmit,
    watch,
    // formState: { errors },
  } = useForm<Inputs>()
  const onSubmit: SubmitHandler<Inputs> = (data) => console.log(data)

  const repeatEvery = watch("repeatEvery");
  console.log("====", repeatEvery)

  return (
      <form onSubmit={handleSubmit(onSubmit)}>

<input type="datetime-local" placeholder="schedule start date"  {...register("scheduledStartDateTime")} />
<input type="number" min={1} defaultValue={1} placeholder="repeat every time" {...register("repeatEveryNumber")}  />
 <select {...register("repeatEvery", { required: true })}>
        <option value={RepeatEvery.Day}>Day</option>
        <option value={RepeatEvery.Week}>week</option>
        <option value={RepeatEvery.Month}>Month</option>
        <option value={RepeatEvery.Year}>year</option>
      </select>

{
 repeatEvery === RepeatEvery.Week && (
  <ToDisplayOnWeek register={register} />
 )
}

{
 repeatEvery === RepeatEvery.Month && (
  <WeekDay name="weekDayForYear" register={register} />
 )
}

<input type="date"  placeholder="schedule end date" {...register("endDate")} />
<button type="submit" >
Submit
</button>
         
      </form>
  )
}


function WeekDay({name, register}: { name: FieldPath<ScheduleDetails>, register: UseFormRegister<ScheduleDetails> }) {
return (
 <select {...register(name, { required: true })}>
        <option value="monday">monday</option>
        <option value="tuesday">tuesday</option>
        <option value="wednesday">wednesday</option>
        <option value="thursday">thursday</option>
        <option value="friday">friday</option>
        <option value="saturday">saturday</option>
        <option value="sunday">sunday</option>
      </select>


)
}


function ToDisplayOnWeek({ register }: { register: UseFormRegister<ScheduleDetails> }) {
  return (
    <div>
      <WeekDay name="weekDayForMonth" register={register} />
    </div>

  )
}