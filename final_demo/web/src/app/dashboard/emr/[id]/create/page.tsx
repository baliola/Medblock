import { redirect } from "next/navigation";
import EMRCreateForm from "@/components/dashboard/emr/create";
import EMRHeader from "@/components/dashboard/emr/header";
import { emrHeader } from "@/constants/contents/dashboard/emr/header";

interface PageProps {
  params: {
    id: string;
  }
}

export default function EMRCreatePage({ params }: PageProps) {
  const id = params.id;

  if (!id) {
    redirect('/dashboard/patients');
  }

  return (
    <EMRCreateForm
      header={
        <EMRHeader title={emrHeader.create.title} />
      }
    />
  )
}