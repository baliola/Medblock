interface VisitSummaryItemProps {
  label: string;
  data: string;
}

const VisitSummaryItem: React.FC<VisitSummaryItemProps> = ({ data, label }) => {
  return (
    <div className="mt-4">
      <p className="text-gray-400 text-xs"> {label} </p>
      <p className="text-gray-800 mt-2"> {data}</p>
    </div>
  );
};

export default VisitSummaryItem;
