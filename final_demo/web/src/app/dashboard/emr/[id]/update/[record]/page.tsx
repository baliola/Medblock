import EMRHeader from "@/components/dashboard/emr/header";
import EMRUpdateForm from "@/components/dashboard/emr/update";
import { emrHeader } from "@/constants/contents/dashboard/emr/header";
import { redirect } from "next/navigation";

interface PageProps {
  params: {
    id: string;
    record: string;
  }
}

const paramFullfilled = (params: PageProps['params']) => {
  if (params.id && params.record) {
    return true;
  }
  return false;
}

export default function EMRUpdatePage({ params }: PageProps) {
  if (!paramFullfilled(params)) {
    redirect('/dashboard/patients');
  }

  return (
    <EMRUpdateForm
      header={
        <EMRHeader title={emrHeader.update.title} />
      }
    />
  )
}