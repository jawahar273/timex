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
  defaultDate?: Date;
};

let allViews = Object.keys(Views).map((k) => Views[k as keyof typeof Views]);

export const DateCalendar = (props: Omit<Props, "localizer">) => {

  return (
    <div>
      <Calendar
        localizer={localizer}
        events={props.cEvents}
        // startAccessor="start"
        // endAccessor="end"
        defaultDate={props.defaultDate}
        step={60}
        view={Views.MONTH}
        showMultiDayTimes
        style={{ height: 500 }}
      />

      <div></div>
    </div>
  );
};
