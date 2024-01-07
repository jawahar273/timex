"use client";

import {
  Calendar,
  DateLocalizer,
  dayjsLocalizer,
  Event,
  Views,
} from "react-big-calendar";
import dayjs from "dayjs";
import { useCallback, useMemo, useState } from "react";
// import  './style.module.css';
import "react-big-calendar/lib/css/react-big-calendar.css";
const localizer = dayjsLocalizer(dayjs);

type temp = typeof Calendar;

type Props = React.ComponentProps<temp> & {
  localizer?: DateLocalizer;
  cEvents: Event[];
};

let allViews = Object.keys(Views).map((k) => Views[k as keyof typeof Views]);

export const DateCalendar = (props: Omit<Props, "localizer">) => {
  // const [view, setView] = useState(Views.MONTH);
  // const onView = useCallback((newView: any) => setView(newView), [])
  // const { defaultDate, max } = useMemo(
  //   () => ({
  //     defaultDate: new Date(2015, 3, 13),
  //     max: dayjs().add(100, 'days').toDate(),
  //   }),
  //   []
  // )

  return (
    <div>
      <Calendar
        localizer={localizer}
        events={props.cEvents}
        // startAccessor="start"
        // endAccessor="end"
        step={60}
        view={Views.MONTH}
        showMultiDayTimes
        style={{ height: 500 }}
      />

      <div></div>
    </div>
  );
};
