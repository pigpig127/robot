import Navbar from "../Navbar";

export default function PcLayout({children}: {children: React.ReactNode}) {
    return (
        <div className='w-full h-full flex flex-row'>
        <Navbar />
        <div className='w-full h-full flex p-6 flex-col box-border'> 
          <div className='h-full w-full rounded-xl overflow-y-auto'>
            {children}
          </div>
        </div>
      </div>
    );
}
  