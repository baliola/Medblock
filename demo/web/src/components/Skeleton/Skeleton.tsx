// React
import { Fragment, PropsWithChildren } from 'react';

function Skeleton(props: PropsWithChildren<{}>) {
  return <Fragment>{props.children}</Fragment>;
}

function Label() {
  return (
    <div className="skeleton h-6  mb-1  bg-gradient-to-r  from-transparent  to-[#CDCED1]  animate-pulse  transition-opacity w-full" />
  );
}
function LabelSuccess() {
  return (
    <div className="skeleton h-6  mb-1  bg-gradient-to-r  from-[#53D258]  to-success  animate-pulse  transition-opacity w-full" />
  );
}

function List() {
  return (
    <div className="skeleton  h-4 bg-gradient-to-r  from-transparent  to-[#CDCED1]  animate-pulse  transition-opacity w-full" />
  );
}

function Card() {
  return (
    <div className="skeleton h-[124px] rounded-lg  mb-1  bg-gradient-to-r  from-transparent  to-[#CDCED1]  animate-pulse  transition-opacity w-full" />
  );
}
function Stats() {
  return (
    <div className="skeleton h-[320px] rounded-lg  mb-1  bg-gradient-to-r  from-transparent  to-[#CDCED1]  animate-pulse  transition-opacity w-full" />
  );
}

Skeleton.Label = Label;
Skeleton.LabelSuccess = LabelSuccess;
Skeleton.Card = Card;
Skeleton.Stats = Stats;
Skeleton.List = List;

export { Skeleton };
