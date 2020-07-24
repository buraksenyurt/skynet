using System;
using System.Linq;
using System.Collections.Generic;
using System.IO;

namespace AdventureWorks.Common
{
    public class BLLInvoice
    {
        [WSATEnable(Active = true, Target = "MSDTC")]
        public bool Accept(Invoice invoice)
        {
            return true;
        }

        [WSATEnable(Active = true, Target = "MSDTC")]
        public bool Reject(int invoiceId)
        {
            return true;
        }
        private Invoice FindInvoice(int invoiceId)
        {
            throw new NotImplementedException();
        }
    }

    public class Invoice
    {
    }

    [AttributeUsage(AttributeTargets.Method)]
    public class WSATEnable
        : Attribute
    {
        public bool Active { get; set; }
        public string Target { get; set; }
    }
}